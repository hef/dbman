use std::hash::Hash;
use std::hash::Hasher;

use k8s_openapi::Metadata;
use k8s_openapi::api::core::v1::ConfigMap;
use k8s_openapi::api::core::v1::Secret;
use kube::Client;

use crate::v1alpha3;
use crate::Error;
use crate::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SourceKind {
    Secret = 0,
    ConfigMap = 2,
}





impl v1alpha3::Credentials {
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
            let mut secret_ref = v1alpha3::SecretRef {
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
        secret_ref: &v1alpha3::SecretRef,
        hasher: &mut dyn Hasher,
    ) -> Result<String> {
        let api = kube::Api::<Secret>::namespaced(client.clone(), namespace);
        let secret = api.get(&secret_ref.name).await?;

        //let z = SourceKind::Secret;
        //z.hash(hasher);
        //SourceKind::Secret.hash(hasher);
        hasher.write(namespace.as_bytes());
        hasher.write(secret_ref.name.as_bytes());
        hasher.write(secret_ref.key.as_bytes());
        // hasher.write);

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
        config_ref: &v1alpha3::ConfigMapRef,
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

    pub fn get_modification_data() {

    }
}
