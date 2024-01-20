use crate::{v1alpha2, Error, Result};
use kube::{Client, ResourceExt};

impl v1alpha2::DatabaseServer {
    pub async fn get_credentials(&self, client: &Client) -> Result<(String, String), Error> {
        let namespace = self
            .namespace()
            .ok_or(Error::MissingNamespace(self.name_any()))?;
        self.spec
            .credentials
            .get_credentials(client, &namespace)
            .await
    }
}
