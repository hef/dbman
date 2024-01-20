use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(
    group = "dbman.hef.sh",
    version = "v1alpha1",
    kind = "DatabaseServer",
    plural = "databaseservers",
    namespaced
)]
pub struct DatabaseServerSpec {
    pub conn_string: String,
    pub superuser_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct DatabaseStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(schema_with = "crate::condition::schema")]
    pub conditions: Vec<Condition>,
}
