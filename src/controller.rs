use crate::{dbc::Dbc, Error, Result};
use k8s_openapi::{
    api::core::v1::{ConfigMap, Secret},
    chrono::{DateTime, Utc},
};
use log::{error, warn};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use futures::StreamExt;
use kube::{
    api::ListParams,
    runtime::{
        controller::Action,
        events::{Event, EventType, Recorder, Reporter},
        finalizer::{finalizer, Event as Finalizer},
        watcher::Config,
        Controller,
    },
    Api, Client, CustomResource, Resource, ResourceExt,
};
use log::info;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub static DATABASE_FINALIZER: &str = "databases.hef.sh/finalizer";

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(
    group = "dbman.hef.sh",
    version = "v1alpha1",
    kind = "Database",
    plural = "databases",
    namespaced
)]
pub struct DatabaseSpec {
    pub database_name: String,
    pub owner: OwnerSource,
    pub password: SecretKeySelector,
    /// should we delete the database when the resource is deleted? Default true
    pub prune: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
pub struct OwnerSource {
    pub config_map_key_ref: Option<ConfigMapKeySelector>,
    pub secret_key_ref: Option<SecretKeySelector>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
pub struct SecretKeySelector {
    pub name: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
pub struct ConfigMapKeySelector {
    pub name: String,
    pub key: String,
}

pub struct DatasbaseStatus {}

#[derive(Clone)]
pub struct Context {
    client: Client,
    conn_string: String,
    diagnostics: Arc<RwLock<Diagnostics>>,
}

impl Context {
    async fn dbc(&self) -> Result<Dbc, tokio_postgres::Error> {
        Dbc::new(&self.conn_string).await
    }
}

async fn reconcile(db: Arc<Database>, ctx: Arc<Context>) -> Result<Action> {
    let ns = db
        .namespace()
        .ok_or(Error::MissingNamespace(db.name_any()))?;
    let dbs: Api<Database> = Api::namespaced(ctx.client.clone(), &ns);

    info!("Reconciling Database \"{}\" in {}", db.name_any(), ns);
    finalizer(&dbs, DATABASE_FINALIZER, db, |event| async {
        match event {
            Finalizer::Apply(db) => db.reconcile(ctx.clone()).await,
            Finalizer::Cleanup(db) => db.cleanup(ctx.clone()).await,
        }
    })
    .await
    .map_err(|e| Error::FinalizerError(Box::new(e)))
}

fn error_policy(_database: Arc<Database>, error: &Error, _ctx: Arc<Context>) -> Action {
    warn!("reconcile failed: {:?}", error);
    Action::requeue(Duration::from_secs(5 * 60))
}

impl Database {
    async fn get_owner(&self, client: &Client) -> Result<String, Error> {
        let namespace = self
            .namespace()
            .ok_or(Error::MissingNamespace(self.name_any()))?;
        if self.spec.owner.config_map_key_ref.is_some() && self.spec.owner.secret_key_ref.is_some()
        {
            return Err(Error::DoNotSpecifyBothSecretAndConfigMap(
                namespace,
                self.name_any(),
            ));
        } else if let Some(config_map_key_selector) = self.spec.owner.config_map_key_ref.as_ref() {
            let config_map_name = config_map_key_selector.name.as_ref();
            let config_map_key: &str = config_map_key_selector.key.as_ref();
            let config_map = Api::<ConfigMap>::namespaced(client.clone(), &namespace)
                .get(config_map_name)
                .await?;
            let owner = config_map
                .data
                .as_ref()
                .ok_or(Error::ConfigMapMissingKey(
                    config_map.name_any(),
                    config_map_key.to_owned(),
                ))?
                .get(config_map_key)
                .ok_or(Error::ConfigMapMissingKey(
                    config_map.name_any(),
                    config_map_key.to_owned(),
                ))?
                .clone();
            return Ok(owner);
        } else if let Some(secret_key_ref) = self.spec.owner.secret_key_ref.as_ref() {
            let secret_name = secret_key_ref.name.as_ref();
            let secret_key: &str = secret_key_ref.key.as_ref();
            let secret = Api::<Secret>::namespaced(client.clone(), &namespace)
                .get(secret_name)
                .await?;

            let owner = String::from_utf8(
                secret
                    .data
                    .as_ref()
                    .ok_or(Error::SecretMissingKey(
                        secret_name.to_owned(),
                        secret_key.to_owned(),
                    ))?
                    .get(secret_key)
                    .ok_or(Error::SecretMissingKey(
                        secret_name.to_owned(),
                        secret_key.to_owned(),
                    ))?
                    .0
                    .clone(),
            )
            .map_err(|_| {
                Error::SecretDidNotContainValidUTF8(secret_name.to_owned(), secret_key.to_owned())
            })?;
            return Ok(owner);
        }
        Err(Error::SpecifyAtLeastOneSecretOrConfigMap)
    }

    async fn get_password(&self, client: &Client) -> Result<String, Error> {
        let namespace = self
            .namespace()
            .ok_or(Error::MissingNamespace(self.name_any()))?;
        let secret_name = self.spec.password.name.as_ref();
        let secret_key: &str = self.spec.password.key.as_ref();
        let secret = Api::<Secret>::namespaced(client.clone(), &namespace)
            .get(secret_name)
            .await?;
        let password = String::from_utf8(
            secret
                .data
                .as_ref()
                .ok_or(Error::SecretMissingKey(
                    secret_name.to_owned(),
                    secret_key.to_owned(),
                ))?
                .get(secret_key)
                .ok_or(Error::SecretMissingKey(
                    secret_name.to_owned(),
                    secret_key.to_owned(),
                ))?
                .0
                .clone(),
        )
        .map_err(|_| {
            Error::SecretDidNotContainValidUTF8(secret_name.to_owned(), secret_key.to_owned())
        })?;
        Ok(password)
    }
    async fn reconcile(&self, ctx: Arc<Context>) -> Result<Action> {
        let dbc = ctx.dbc().await?;
        let client = ctx.client.clone();
        let recorder = ctx.diagnostics.read()?.recorder(client.clone(), self);

        let owner = self.get_owner(&client).await?;

        if !dbc.does_user_exist(&owner).await? {
            let password = self.get_password(&client).await?;
            recorder
                .publish(Event {
                    type_: EventType::Normal,
                    reason: "CreatingUser".into(),
                    note: Some(format!("Creating user `{owner}`")),
                    action: "Creating User".into(),
                    secondary: None,
                })
                .await?;
            dbc.create_user(&owner).await?;
            recorder
                .publish(Event {
                    type_: EventType::Normal,
                    reason: "UpdatingPassword".into(),
                    note: Some(format!("Updating password for `{owner}`")),
                    action: "Updating Password".into(),
                    secondary: None,
                })
                .await?;
            dbc.update_password(&owner, &password).await?;
        }

        let database_name = &self.spec.database_name;
        if !dbc.does_database_exist(database_name).await? {
            let owner = self.get_owner(&client).await?;
            recorder
                .publish(Event {
                    type_: EventType::Normal,
                    reason: "CreatingDatabase".into(),
                    note: Some(format!("Creating database `{database_name}`")),
                    action: "Creating Database".into(),
                    secondary: None,
                })
                .await?;
            dbc.create_database(owner.as_ref(), database_name).await?;
            recorder.publish(Event {
                type_: EventType::Normal,
                reason: "GrantingPrivileges".into(),
                note: Some(format!("Granting privileges on database `{database_name}` to `{owner}`")),
                action: "Granting Privileges".into(),
                secondary: None,
            }).await?;
            dbc.grant_all_privileges_on_database_to_user(database_name, &owner)
                .await?;
        }

        Ok(Action::requeue(Duration::from_secs(5 * 60)))
    }

    async fn cleanup(&self, ctx: Arc<Context>) -> Result<Action> {
        let dbc = ctx.dbc().await?;
        let client = ctx.client.clone();
        let recorder = ctx.diagnostics.read()?.recorder(client.clone(), self);
        if self.spec.prune.unwrap_or(true) {
            let database_name = &self.spec.database_name;
            recorder.publish(Event {
                type_: EventType::Normal,
                reason: "DroppingDatabase".into(),
                note: Some(format!("Dropping database `{database_name}`")),
                action: "Dropping Database".into(),
                secondary: None,
            }).await?;
            dbc.drop_database(database_name).await?;
            let owner = self.get_owner(&ctx.client).await?;
            recorder.publish( Event {
                type_: EventType::Normal,
                reason: "DroppingUser".into(),
                note: Some(format!("Dropping user `{owner}`")),
                action: "Dropping User".into(),
                secondary: None,
            }).await?;
            dbc.drop_user(owner.as_ref()).await?;
        }
        Ok(Action::await_change())
    }
}

#[derive(Clone, Serialize)]
pub struct Diagnostics {
    #[serde(deserialize_with = "from_ts")]
    pub last_event: DateTime<Utc>,
    #[serde(skip)]
    pub reporter: Reporter,
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self {
            last_event: Utc::now(),
            reporter: "dbman".into(),
        }
    }
}

impl Diagnostics {
    fn recorder(&self, client: Client, db: &Database) -> Recorder {
        Recorder::new(client, self.reporter.clone(), db.object_ref(&()))
    }
}

#[derive(Clone, Default)]
pub struct State {
    pub conn_string: String,
    pub diangostics: Arc<RwLock<Diagnostics>>,
}

impl State {
    pub fn to_context(&self, client: Client) -> Arc<Context> {
        Arc::new(Context {
            client,
            conn_string: self.conn_string.clone(),
            diagnostics: self.diangostics.clone(),
        })
    }
}

pub async fn run(state: State) -> Result<(), Error> {
    let client = Client::try_default().await?;
    let databases = Api::<Database>::all(client.clone());

    if let Err(e) = databases.list(&ListParams::default().limit(1)).await {
        error!("CRD is not queryable; {e:?}. Is the CRD installed?");
        info!("Installation: cargo run --bin crdgen | kubectl apply -f -");
        std::process::exit(1);
    }

    Controller::new(databases, Config::default().any_semantic())
        .shutdown_on_signal()
        .run(reconcile, error_policy, state.to_context(client))
        .filter_map(|x| async move { std::result::Result::ok(x) })
        .for_each(|_| futures::future::ready(()))
        .await;
    Ok(())
}
