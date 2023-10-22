use anyhow::Context;
use controller::State;
use k8s_openapi::{
    api::{core::v1::{Pod, Secret}, apps::v1::Deployment},
    apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
};
use kube::{
    api::{DeleteParams, PostParams},
    core::{DynamicObject, GroupVersionKind, ObjectMeta, TypeMeta},
    runtime::{
        conditions::{self, is_pod_running},
        wait::await_condition,
    },
    Api, CustomResourceExt, Discovery,
};
use log::info;
use serde_json::json;
use std::{net::SocketAddr, collections::BTreeMap};
use tokio_postgres::NoTls;
mod common;
use futures::{StreamExt, TryStreamExt};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;

#[tokio::test]
async fn test_x() {
    let client = common::get_kube_client().await;

    let discovery = Discovery::new(client.clone()).run().await.unwrap();

    let yaml_text = reqwest::get("https://raw.githubusercontent.com/cloudnative-pg/cloudnative-pg/release-1.20/releases/cnpg-1.20.2.yaml").await.unwrap().text().await.unwrap();
    common::kubectl_apply(&client, &yaml_text).await;
    common::kubectl_apply(
        &client,
        &serde_yaml::to_string(&controller::Database::crd()).unwrap(),
    )
    .await;
    common::kubectl_apply(
        &client,
        &serde_yaml::to_string(&controller::DatabaseServer::crd()).unwrap(),
    )
    .await;

    let cluster = DynamicObject {
        types: Some(TypeMeta {
            api_version: "postgresql.cnpg.io/v1".into(),
            kind: "Cluster".into(),
        }),
        metadata: ObjectMeta {
            name: Some("db".into()),
            namespace: Some("default".into()),
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

    let crds: Api<CustomResourceDefinition> = Api::all(client.clone());
    let establish = await_condition(
        crds,
        "clusters.postgresql.cnpg.io",
        conditions::is_crd_established(),
    );
    let _ = tokio::time::timeout(std::time::Duration::from_secs(10), establish)
        .await
        .unwrap();

    let discovery = discovery.run().await.unwrap();
    let gvk = GroupVersionKind::try_from(&cluster.types.clone().unwrap()).unwrap();
    let (api_resource, _api_capabilities) = discovery.resolve_gvk(&gvk).unwrap();

    let deployment_api = Api::<Deployment>::namespaced(client.clone(), "cnpg-system");
    let available = await_condition(deployment_api, "cnpg-controller-manager", common::is_deployment_available());
    let _ = tokio::time::timeout(std::time::Duration::from_secs(30), available)
        .await
        .unwrap()
        .unwrap(); // wait for cnpg to start, otherwise create hooks can fail a webhook

    let api: Api<DynamicObject> = Api::namespaced_with(
        client.clone(),
        &cluster.metadata.namespace.clone().unwrap(),
        &api_resource,
    );

    match api.delete("db", &DeleteParams::default()).await {
        Ok(_) => info!("deleted"),
        Err(_) => info!("db not found"),
    };

    api.create(&PostParams::default(), &cluster).await.unwrap();

    let pods: Api<Pod> = Api::namespaced(client.clone(), "default");
    let running = await_condition(pods.clone(), "db-1", is_pod_running());
    let _ = tokio::time::timeout(std::time::Duration::from_secs(30), running)
        .await
        .unwrap()
        .unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 5432));

    tokio::spawn(async move {
        let server = TcpListenerStream::new(TcpListener::bind(addr).await.unwrap())
            .take_until(tokio::signal::ctrl_c())
            .try_for_each(|mut client_conn| async {
                let pods = pods.clone();
                tokio::spawn(async move {
                    let mut forwarder = pods.portforward("db-1", &[5432]).await.unwrap();
                    let mut upstream_conn = forwarder.take_stream(5432).context("x").unwrap();
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

    // the db-superuser secret won't exist until this is done
    let db_ready = await_condition(api.clone(), "db", common::is_cluster_ready());
    let _ = tokio::time::timeout(std::time::Duration::from_secs(30), db_ready)
        .await
        .unwrap()
        .unwrap();

    let secrets: Api<Secret> = Api::<Secret>::namespaced(client.clone(), "default");
    let secret = secrets.get("db-superuser").await.unwrap(); // todo: wait for secret to exist
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

    let (pg_client, conn) = tokio_postgres::connect(
        &format!("host=localhost user={username} password={password}"),
        NoTls,
    )
    .await
    .unwrap();

    tokio::spawn(async move {
        conn.await.unwrap();
    });

    let result = pg_client
        .query(
            "select 1 from pg_database where datname = $1::TEXT",
            &[&"my_db"],
        )
        .await
        .unwrap();
    assert_eq!(result.len(), 0);

    let dbs = controller::DatabaseServer {
        metadata: ObjectMeta {
            name: Some("dbs".into()),
            namespace: Some("default".into()),
            ..Default::default()
        },
        spec: controller::DatabaseServerSpec {
            conn_string: "host=localhost".into(),
            superuser_secret: "db-superuser".into(),
        },
    };

    let dbs_api = Api::<controller::DatabaseServer>::namespaced(client.clone(), "default");
    dbs_api.create(&PostParams::default(), &dbs).await.ok();

    let db = controller::Database {
        metadata: ObjectMeta {
            name: Some("my-db".into()),
            namespace: Some("default".into()),
            ..Default::default()
        },
        spec: controller::DatabaseSpec {
            database_server_ref: controller::DatabaseServerRef {
                name: "dbs".into(),
                namespace: Some("default".into()),
            },
            database_name: "my-db".into(),
            credentials_secret: "my-db-credentials".into(),
            prune: Some(false),
        },
        status: Some(controller::DatabaseStatus {
            conditions: vec![]
        })
    };


    let secret_api = Api::<Secret>::namespaced(client.clone(), "default");


    let mut creds: BTreeMap<String, String> = BTreeMap::new();
    creds.insert("username".into(),"my-username".into());
    creds.insert("password".into(),"my-password".into());

    secret_api
        .create(
            &PostParams::default(),
            &Secret {
                metadata: ObjectMeta {
                    name: Some("my-db-credentials".into()),
                    namespace: Some("default".into()),
                    ..Default::default()
                },
                type_: Some("kubernetes.io/basic-auth".into()),
                string_data: Some(creds),
                ..Default::default()
            },
        )
        .await
        .ok();


    tokio::spawn(controller::run(State::default()));


    let db_api = Api::<controller::Database>::namespaced(client.clone(), "default");    
    db_api.create(&PostParams::default(), &db).await.ok();

    let db_ready = await_condition(db_api.clone(), "my-db", common::is_database_ready());
    let _ = tokio::time::timeout(std::time::Duration::from_secs(30), db_ready)
        .await
        .unwrap()
        .unwrap();

    let result = pg_client
        .query(
            "select 1 from pg_database where datname = $1::TEXT",
            &[&"my-db"],
        )
        .await
        .unwrap();
    assert_eq!(result.len(), 1);
}
