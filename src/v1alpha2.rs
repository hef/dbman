use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::v1alpha3;

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(
    group = "dbman.hef.sh",
    version = "v1alpha2",
    kind = "Database",
    plural = "databases",
    status = "DatabaseStatus",
    namespaced
)]
pub struct DatabaseSpec {
    pub database_server_ref: DatabaseServerRef,
    pub database_name: String,
    pub credentials_secret: String,
    /// should we delete the database when the resource is deleted? Default true
    pub prune: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
pub struct DatabaseServerRef {
    pub name: String,
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct DatabaseStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(schema_with = "crate::condition::schema")]
    pub conditions: Vec<Condition>,
}

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(
    group = "dbman.hef.sh",
    version = "v1alpha2",
    kind = "DatabaseServer",
    plural = "databaseservers",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseServerSpec {
    pub conn_string: String,
    pub credentials: v1alpha3::Credentials,
}
