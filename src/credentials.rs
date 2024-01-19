use k8s_openapi::api::core::v1::ConfigMap;
use k8s_openapi::api::core::v1::Secret;
use kube::Client;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Error;
use crate::Result;

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
pub struct ConfigMapRef {
    pub name: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
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

impl Credentials {
    fn validate(&self) -> Result<(), Error> {
        if self.basic_auth_secret_ref.is_some()
            && (self.username.is_some()
                || self.username_config_map_ref.is_some()
                || self.username_secret_ref.is_some()
                || self.password_secret_ref.is_some())
        {
            return Err(Error::BasicAuthSecretRefIsMutuallyExclusiveWithOtherCredentialFields());
        }
        if self.username.is_some()
            && (self.username_config_map_ref.is_some() || self.username_secret_ref.is_some())
        {
            return Err(Error::UsernameFieldIsMutuallyExclusiveWithUsernameConfigRefAndUsernameSecretRefFields());
        }
        if self.username_config_map_ref.is_some() && (self.username_secret_ref.is_some()) {
            return Err(Error::UsernameConfigRefAndUsernameSecretRefFieldsAreMutuallyExclusive());
        }

        Ok(())
    }

    pub async fn get_credentials(
        &self,
        client: &Client,
        namespace: &str,
    ) -> Result<(String, String)> {
        self.validate()?;
        if let Some(basic_auth_secret_ref) = self.basic_auth_secret_ref.clone() {
            let mut secret_ref = SecretRef {
                name: basic_auth_secret_ref,
                key: "username".into(),
            };
            let username = self
                .get_secret_value(client, namespace, &secret_ref)
                .await?;
            secret_ref.key = "password".into();
            let password = self
                .get_secret_value(client, namespace, &secret_ref)
                .await?;
            return Ok((username, password));
        }
        if let Some(username) = self.username.clone() {
            if let Some(password_secret_ref) = self.password_secret_ref.clone() {
                let password = self
                    .get_secret_value(client, namespace, &password_secret_ref)
                    .await?;
                return Ok((username, password));
            }
            return Err(Error::PasswordSecretRefFieldIsRequiredWhenUsernameFieldIsSet());
        }
        if let Some(username_config_ref) = self.username_config_map_ref.clone() {
            let username = Self::get_config_value(client, namespace, &username_config_ref).await?;
            if let Some(password_secret_ref) = self.password_secret_ref.clone() {
                let password = self
                    .get_secret_value(client, namespace, &password_secret_ref)
                    .await?;
                return Ok((username, password));
            }
            return Err(Error::PasswordSecretRefFieldIsRequiredWhenUsernameConfigRefFieldIsSet());
        }
        if let Some(username_secret_ref) = self.username_secret_ref.clone() {
            let username = self
                .get_secret_value(client, namespace, &username_secret_ref)
                .await?;
            if let Some(password_secret_ref) = self.password_secret_ref.clone() {
                let password = self
                    .get_secret_value(client, namespace, &password_secret_ref)
                    .await?;
                return Ok((username, password));
            }
            return Err(Error::PasswordSecretRefFieldIsRequiredWhenUsernameSecretRefFieldIsSet());
        }
        Ok(("".into(), "".into()))
    }

    async fn get_secret_value(
        &self,
        client: &Client,
        namespace: &str,
        secret_ref: &SecretRef,
    ) -> Result<String> {
        let api = kube::Api::<Secret>::namespaced(client.clone(), namespace);
        let secret = api.get(&secret_ref.name).await?;
        let byte_value = secret
            .data
            .as_ref()
            .ok_or(Error::SecretMissingKey(
                secret_ref.name.clone(),
                secret_ref.key.clone(),
            ))?
            .get(&secret_ref.key)
            .ok_or(Error::SecretMissingKey(
                secret_ref.name.clone(),
                secret_ref.key.clone(),
            ))?
            .clone();
        let value = String::from_utf8(byte_value.0).map_err(|e| {
            Error::SecretDidNotContainValidUTF8(secret_ref.name.clone(), e.to_string())
        })?;
        Ok(value)
    }

    async fn get_config_value(
        client: &Client,
        namespace: &str,
        config_ref: &ConfigMapRef,
    ) -> Result<String> {
        let api = kube::Api::<ConfigMap>::namespaced(client.clone(), namespace);
        let config_map = api.get(&config_ref.name).await?;
        let value = config_map
            .data
            .as_ref()
            .ok_or(Error::ConfigMapMissingKey(
                config_ref.name.clone(),
                config_ref.key.clone(),
            ))?
            .get(&config_ref.key)
            .ok_or(Error::ConfigMapMissingKey(
                config_ref.name.clone(),
                config_ref.key.clone(),
            ))?
            .clone();
        Ok(value)
    }
}
