use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(
    group = "dbman.hef.sh",
    version = "v1alpha3",
    kind = "Database",
    plural = "databases",
    status = "DatabaseStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseSpec {
    pub database_server_ref: DatabaseServerRef,
    pub database_name: String,
    pub credentials: Option<Credentials>,
    pub owner_ref: Option<String>, // todo: credentials, credentials_secret, and owner_ref are mutually exclusive
    /// should we delete the database when the resource is deleted? Default true
    pub prune: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseServerRef {
    pub name: String,
    pub namespace: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(schema_with = "crate::condition::schema")]
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapRef {
    pub name: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SecretRef {
    pub name: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub basic_auth_secret_ref: Option<String>,
    pub username: Option<String>,
    pub username_config_map_ref: Option<ConfigMapRef>,
    pub username_secret_ref: Option<SecretRef>,
    pub password_secret_ref: Option<SecretRef>,
}
