use kube::{CustomResource, Client, ResourceExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{credentials::Credentials, Error, Result};

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
    pub credentials: Credentials,
}

impl DatabaseServer {
    pub async fn get_credentials(&self, client: &Client) -> Result<(String, String), Error> {
        let namespace = self
            .namespace()
            .ok_or(Error::MissingNamespace(self.name_any()))?;
        self.spec.credentials.get_credentials(client, &namespace).await
    }
}