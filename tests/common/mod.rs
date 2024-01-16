use controller::Database;
use futures::{StreamExt, TryStreamExt};
use k8s_openapi::{
    api::{
        apps::v1::Deployment,
        core::v1::{Namespace, Pod, Secret},
    },
    apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
};
use kube::{
    api::{DeleteParams, Patch, PatchParams, PostParams},
    config::{KubeConfigOptions, Kubeconfig},
    core::{DynamicObject, GroupVersionKind, ObjectMeta, TypeMeta},
    discovery::Scope,
    runtime::{
        conditions::{self, is_deleted, is_pod_running},
        wait::{await_condition, Condition},
    },
    Api, Client, Config, CustomResourceExt, Discovery, ResourceExt,
};
use serde_json::json;
use std::{collections::BTreeMap, net::SocketAddr};
use tokio::{net::TcpListener, process::Command};
use tokio_postgres::{tls::NoTlsStream, NoTls, Socket};
use tokio_stream::wrappers::TcpListenerStream;

pub async fn get_kube_client() -> Client {
    let kubeconfig_raw = Command::new("kind")
        .args(["get", "kubeconfig"])
        .output()
        .await
        .expect("failed to get kubeconfig")
        .stdout;
    let kubeconfig_string = String::from_utf8(kubeconfig_raw).expect("failed to get kubeconfig");
    let kubeconfig = Kubeconfig::from_yaml(&kubeconfig_string).unwrap();
    let config = Config::from_custom_kubeconfig(kubeconfig, &KubeConfigOptions::default())
        .await
        .unwrap();

    Client::try_from(config).unwrap()
}

pub async fn kubectl_apply(client: &Client, yaml_text: &str) {
    let discovery = Discovery::new(client.clone()).run().await.unwrap();
    let pp: PatchParams = PatchParams::apply("dbman-test").force();

    //let de = serde_yaml::Deserializer::from_str(&yaml_text);
    use serde::Deserialize;
    for de in serde_yaml::Deserializer::from_str(yaml_text) {
        let item = serde_yaml::Value::deserialize(de).unwrap();
        let obj: DynamicObject = serde_yaml::from_value(item).unwrap();
        let namespace = obj.metadata.namespace.as_deref().unwrap_or("default");
        let gvk = GroupVersionKind::try_from(&obj.clone().types.unwrap()).unwrap();

        let name = obj.name_any();

        let (api_resource, api_capabilities) = discovery.resolve_gvk(&gvk).unwrap();

        let api: Api<DynamicObject> = if api_capabilities.scope == Scope::Cluster {
            Api::all_with(client.clone(), &api_resource)
        } else {
            //Api::default_namespaced_with(client.clone(), &apiResource)
            Api::namespaced_with(client.clone(), namespace, &api_resource)
        };
        let data: serde_json::Value = serde_json::to_value(&obj).unwrap();

        api.patch(&name, &pp, &Patch::Apply(data)).await.unwrap();
    }
    // we might have installed cluster crds for the first time, run discovery again
    //let discovery = Discovery::new(client.clone()).run().await.unwrap();
}

pub async fn setup_cnpg(client: &Client) {
    let yaml_text = reqwest::get("https://raw.githubusercontent.com/cloudnative-pg/cloudnative-pg/release-1.20/releases/cnpg-1.20.2.yaml").await.unwrap().text().await.unwrap();
    kubectl_apply(client, &yaml_text).await;

    let crds: Api<CustomResourceDefinition> = Api::all(client.clone());
    let establish = await_condition(
        crds,
        "clusters.postgresql.cnpg.io",
        conditions::is_crd_established(),
    );

    let _ = tokio::time::timeout(std::time::Duration::from_secs(10), establish)
        .await
        .unwrap();

    let deployment_api = Api::<Deployment>::namespaced(client.clone(), "cnpg-system");
    let available = await_condition(
        deployment_api,
        "cnpg-controller-manager",
        is_deployment_available(),
    );
    let _ = tokio::time::timeout(std::time::Duration::from_secs(30), available)
        .await
        .unwrap()
        .unwrap(); // wait for cnpg to start, otherwise create hooks can fail a webhook
}

pub async fn instal_crds(client: &Client) {
    kubectl_apply(
        client,
        &serde_yaml::to_string(&controller::Database::crd()).unwrap(),
    )
    .await;
    kubectl_apply(
        client,
        &serde_yaml::to_string(&controller::DatabaseServer::crd()).unwrap(),
    )
    .await;
    let crds: Api<CustomResourceDefinition> = Api::all(client.clone());
    let establish = await_condition(
        crds,
        "clusters.postgresql.cnpg.io",
        conditions::is_crd_established(),
    );
    let _ = tokio::time::timeout(std::time::Duration::from_secs(10), establish)
        .await
        .unwrap();
}

pub struct DatabaseServerHandle {
    namespace: String,
    #[allow(dead_code)] // #[expect(dead_code, reason = "port_forwarder is a handle that we drop")]
    port_forwarder: tokio::task::JoinHandle<()>,
    port: u16,
}

impl DatabaseServerHandle {
    pub async fn new(client: &Client, namespace: String) -> Self {
        let cluster = DynamicObject {
            types: Some(TypeMeta {
                api_version: "postgresql.cnpg.io/v1".into(),
                kind: "Cluster".into(),
            }),
            metadata: ObjectMeta {
                name: Some("db".into()),
                namespace: Some(namespace.clone()),
                ..Default::default()
            },
            data: json!({
                "spec": {
                    "instances": 1,
                    "primaryUpdateStrategy": "unsupervised",
                    "storage": {
                        "size": "1Gi"
                    }
                }
            }),
        };

        let discovery = Discovery::new(client.clone()).run().await.unwrap();
        let gvk = GroupVersionKind::try_from(&cluster.types.clone().unwrap()).unwrap();
        let (api_resource, _api_capabilities) = discovery.resolve_gvk(&gvk).unwrap();

        let api: Api<DynamicObject> = Api::namespaced_with(
            client.clone(),
            &cluster.metadata.namespace.clone().unwrap(),
            &api_resource,
        );

        api.create(&PostParams::default(), &cluster).await.unwrap();

        let pods: Api<Pod> = Api::namespaced(client.clone(), &namespace);
        let running = await_condition(pods.clone(), "db-1", is_pod_running());
        let _ = tokio::time::timeout(std::time::Duration::from_secs(30), running)
            .await
            .unwrap()
            .unwrap();

        let addr = SocketAddr::from(([127, 0, 0, 1], 0));
        let listener = TcpListener::bind(addr).await.unwrap();
        let port = listener.local_addr().unwrap().port();

        let port_forwarder = tokio::spawn(async move {
            let server = TcpListenerStream::new(listener)
                .take_until(tokio::signal::ctrl_c())
                .try_for_each(|mut client_conn| async {
                    let pods = pods.clone();
                    tokio::spawn(async move {
                        let mut forwarder = pods.portforward("db-1", &[5432]).await.unwrap();
                        let mut upstream_conn = forwarder.take_stream(5432).unwrap();
                        tokio::io::copy_bidirectional(&mut client_conn, &mut upstream_conn)
                            .await
                            .unwrap();
                        drop(upstream_conn);
                        forwarder.join().await.unwrap();
                        //info!("connection closed");
                    });
                    Ok(())
                });
            server.await.unwrap();
        });

        Self {
            port_forwarder,
            port,
            namespace,
        }
    }

    pub async fn connect(
        &self,
        client: &Client,
    ) -> (
        tokio_postgres::Client,
        tokio_postgres::Connection<Socket, NoTlsStream>,
    ) {
        let (username, password) =
            get_credentials_from_secret(client, &self.namespace, "db-superuser").await;

        let port = self.port;
        let (pg_client, conn) = tokio_postgres::connect(
            &format!("host=localhost port={port} user={username} password={password}"),
            NoTls,
        )
        .await
        .unwrap();

        (pg_client, conn)
    }

    pub async fn create_database_server(&self, client: &Client, namespace: String, name: String) {
        let port = self.port;
        let dbs = controller::DatabaseServer {
            metadata: ObjectMeta {
                name: Some(name),
                namespace: Some(namespace.clone()),
                ..Default::default()
            },
            spec: controller::DatabaseServerSpec {
                conn_string: format!("host=localhost port={port}"),
                superuser_secret: "db-superuser".into(),
            },
        };

        let dbs_api = Api::<controller::DatabaseServer>::namespaced(client.clone(), &namespace);
        dbs_api.create(&PostParams::default(), &dbs).await.unwrap();
    }
    pub fn get_port(&self) -> u16 {
        self.port
    }
}

#[must_use]
pub fn is_database_ready() -> impl Condition<Database> {
    |obj: Option<&Database>| {
        if let Some(database) = &obj {
            if let Some(status) = database.status.as_ref() {
                if let Some(condition) = status.conditions.iter().find(|c| c.type_ == "Ready") {
                    return condition.status == "True";
                }
            }
        }
        false
    }
}

#[must_use]
pub fn is_deployment_available() -> impl Condition<Deployment> {
    |obj: Option<&Deployment>| {
        if let Some(deployment) = &obj {
            if let Some(status) = deployment.status.as_ref() {
                if let Some(condition) = status
                    .conditions
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|c| c.type_ == "Available")
                {
                    return condition.status == "True";
                }
            }
        }
        false
    }
}

pub async fn delete_db_object(api: &Api<Database>, db_object: &Database) {
    let dbname = db_object.name_any();
    api.delete(dbname.as_str(), &DeleteParams::default())
        .await
        .unwrap();
    let uid = db_object.uid().unwrap();
    let deleted = await_condition(api.clone(), dbname.as_str(), is_deleted(&uid));
    let _ = tokio::time::timeout(std::time::Duration::from_secs(30), deleted)
        .await
        .unwrap()
        .expect("timed out deleting api");
}

pub struct ScopedNamespace {
    client: Client,
    pub name: String,
}

impl ScopedNamespace {
    pub async fn new(client: Client, name: String) -> Self {
        let api = Api::<Namespace>::all(client.clone());
        let create_result = api
            .create(
                &PostParams::default(),
                &Namespace {
                    metadata: ObjectMeta {
                        name: Some(name.clone()),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .await;

        match create_result {
            Ok(_) => {}
            Err(e) => {
                match e {
                    kube::Error::Api(kube_error) => {
                        if kube_error.code == 409 {
                            // namespace already exists, delete and recreate
                            let namespace_object = api.get(&name).await.unwrap();
                            api.delete(
                                &name,
                                &DeleteParams {
                                    grace_period_seconds: Some(0),
                                    ..Default::default()
                                },
                            )
                            .await
                            .unwrap();
                            let uid = namespace_object.clone().uid().unwrap();
                            let deleted = await_condition(api.clone(), &name, is_deleted(&uid));
                            let _ =
                                tokio::time::timeout(std::time::Duration::from_secs(300), deleted)
                                    .await
                                    .unwrap()
                                    .unwrap();

                            api.create(
                                &PostParams::default(),
                                &Namespace {
                                    metadata: ObjectMeta {
                                        name: Some(name.clone()),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                            )
                            .await
                            .unwrap();
                        } else {
                            panic!("unexpected error: {:?}", kube_error);
                        }
                    }
                    _ => panic!("unexpected error: {:?}", e),
                }
            }
        }

        Self { client, name }
    }
}

impl Drop for ScopedNamespace {
    fn drop(&mut self) {
        let api = Api::<Namespace>::all(self.client.clone());
        let name = self.name.clone();
        tokio::spawn(async move { api.delete(&name, &Default::default()).await });
    }
}

pub async fn get_credentials_from_secret(
    client: &Client,
    namespace: &str,
    name: &str,
) -> (String, String) {
    let secrets: Api<Secret> = Api::<Secret>::namespaced(client.clone(), namespace);
    let secret = secrets.get(name).await.unwrap(); // todo: wait for secret to exist
    let username = String::from_utf8(
        secret
            .data
            .clone()
            .unwrap()
            .get("username")
            .unwrap()
            .clone()
            .0,
    )
    .unwrap();
    let password =
        String::from_utf8(secret.data.unwrap().get("password").unwrap().clone().0).unwrap();

    (username, password)
}

pub async fn store_credentials_in_secret(
    client: &Client,
    namespace: String,
    name: String,
    username: String,
    password: String,
) {
    let secret_api = Api::<Secret>::namespaced(client.clone(), &namespace);

    let mut creds: BTreeMap<String, String> = BTreeMap::new();
    creds.insert("username".into(), username);
    creds.insert("password".into(), password);

    secret_api
        .create(
            &PostParams::default(),
            &Secret {
                metadata: ObjectMeta {
                    name: Some(name),
                    namespace: Some(namespace),
                    ..Default::default()
                },
                type_: Some("kubernetes.io/basic-auth".into()),
                string_data: Some(creds),
                ..Default::default()
            },
        )
        .await
        .unwrap();
}

pub async fn does_pgdatabase_exist(dbc: &tokio_postgres::Client, dbname: &String ) -> bool {
    let result = dbc
        .query(
            "select 1 from pg_database where datname = $1::TEXT",
            &[dbname],
        )
        .await
        .unwrap();
    result.len() == 1
}