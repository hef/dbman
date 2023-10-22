use controller::Database;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{
    api::{Patch, PatchParams},
    config::{KubeConfigOptions, Kubeconfig},
    core::{DynamicObject, GroupVersionKind},
    discovery::Scope,
    runtime::wait::Condition,
    Api, Client, Config, Discovery, ResourceExt,
};
use tokio::process::Command;

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

#[must_use]
pub fn is_cluster_ready() -> impl Condition<DynamicObject> {
    |obj: Option<&DynamicObject>| {
        if let Some(cluster) = &obj {
            if let Some(status) = cluster.data.get("status") {
                if let Some(phase) = status.get("phase") {
                    return phase.as_str().unwrap() == "Cluster in healthy state";
                }
            }
        }
        false
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
                if let Some(condition) = status.conditions.as_ref().unwrap().iter().find(|c| c.type_ == "Available") {
                    return condition.status == "True";
                }
            }
        }
        false
    }
}
