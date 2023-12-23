use crate::{
    condition::{Reason, Status, Type},
    dbc::Dbc,
    Error, Result,
};
use k8s_openapi::{
    api::core::v1::Secret,
    apimachinery::pkg::apis::meta::v1::{Condition, Time},
    chrono::{DateTime, Utc},
};
use log::{error, warn};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use futures::StreamExt;
use kube::{
    api::{ListParams, Patch, PatchParams},
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
use crate::heritage::Heritage;

pub static DATABASE_FINALIZER: &str = "databases.hef.sh/finalizer";

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

#[derive(Clone)]
pub struct Context {
    client: Client,
    diagnostics: Arc<RwLock<Diagnostics>>,
}

impl Context {}

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
    async fn dbc(&self, client: &Client) -> Result<Dbc> {
        //todo:
        let database_server_namespace = self
            .spec
            .database_server_ref
            .namespace
            .as_ref()
            .or(self.namespace().as_ref())
            .ok_or(Error::MissingNamespace(self.name_any()))?
            .to_owned();

        let api: Api<DatabaseServer> = Api::namespaced(client.clone(), &database_server_namespace);
        let dbs: DatabaseServer = api.get(&self.spec.database_server_ref.name).await?;
        let (superuser_name, superuser_password) = dbs.get_credentials(client).await?;
        let dbc = Dbc::new(&dbs.spec.conn_string, &superuser_name, &superuser_password).await?;
        Ok(dbc)
    }
    async fn get_credentials(&self, client: &Client) -> Result<(String, String), Error> {
        let namespace = self
            .namespace()
            .ok_or(Error::MissingNamespace(self.name_any()))?;
        let secret_name = self.spec.credentials_secret.as_ref();
        let secret = Api::<Secret>::namespaced(client.clone(), &namespace)
            .get(secret_name)
            .await?;

        let username = String::from_utf8(
            secret
                .data
                .as_ref()
                .ok_or(Error::SecretMissingKey(
                    secret_name.to_owned(),
                    "username".to_owned(),
                ))?
                .get("username")
                .ok_or(Error::SecretMissingKey(
                    secret_name.to_owned(),
                    "username".to_owned(),
                ))?
                .0
                .clone(),
        )
        .map_err(|e| Error::SecretDidNotContainValidUTF8(secret_name.to_owned(), e.to_string()))?;

        let password = String::from_utf8(
            secret
                .data
                .as_ref()
                .ok_or(Error::SecretMissingKey(
                    secret_name.to_owned(),
                    "password".to_owned(),
                ))?
                .get("password")
                .ok_or(Error::SecretMissingKey(
                    secret_name.to_owned(),
                    "password".to_owned(),
                ))?
                .0
                .clone(),
        )
        .map_err(|e| Error::SecretDidNotContainValidUTF8(secret_name.to_owned(), e.to_string()))?;

        Ok((username, password))
    }

    async fn reconcile(&self, ctx: Arc<Context>) -> Result<Action> {
        let client = ctx.client.clone();
        let dbc = self.dbc(&client).await?;
        let recorder = ctx.diagnostics.read()?.recorder(client.clone(), self);

        /*self.set_condition(
            &client,
            Type::Ready,
            Status::False,
            Reason::Initializing,
            "Initializing",
        )
        .await?;*/

        let (owner, password) = self.get_credentials(&client).await?;

        if !dbc.does_user_exist(&owner).await? {
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
            let heritage = Heritage::builder().owner(owner).resource(&self).build();
            dbc.apply_heritage(database_name, &heritage).await?;
            recorder
                .publish(Event {
                    type_: EventType::Normal,
                    reason: "GrantingPrivileges".into(),
                    note: Some(format!(
                        "Granting privileges on database `{database_name}` to `{owner}`"
                    )),
                    action: "Granting Privileges".into(),
                    secondary: None,
                })
                .await?;
            dbc.validate_heritage(database_name, &heritage).await?;
            dbc.grant_all_privileges_on_database_to_user(database_name, &owner)
                .await?;
        }

        self.set_condition(&client, Type::Ready, Status::True, Reason::Success, "")
            .await?;

        Ok(Action::requeue(Duration::from_secs(5 * 60)))
    }

    async fn set_condition(
        &self,
        client: &Client,
        type_: Type,
        status: Status,
        reason: Reason,
        message: &str,
    ) -> Result<()> {
        let condition = Condition {
            last_transition_time: Time(Utc::now()),
            message: message.to_owned(),
            observed_generation: self.metadata.generation,
            reason: reason.to_string(),
            status: status.to_string(),
            type_: type_.to_string(),
        };

        // todo: use Patch::Apply for server side apply
        let patch = Patch::Merge(serde_json::json!({
            "status": {
                "conditions": [condition]
            }
        }));
        let api = Api::<Database>::namespaced(
            client.clone(),
            &self
                .namespace()
                .ok_or(Error::MissingNamespace(self.name_any()))?,
        );

        //let pp = PatchParams::apply("dbman-condition");
        let pp = PatchParams::default();

        api.patch_status(&self.name_any(), &pp, &patch).await?;
        Ok(())
    }

    async fn cleanup(&self, ctx: Arc<Context>) -> Result<Action> {
        let client = ctx.client.clone();
        let dbc = self.dbc(&client).await?;
        let recorder = ctx.diagnostics.read()?.recorder(client.clone(), self);
        if self.spec.prune.unwrap_or(true) {
            let database_name = &self.spec.database_name;
            recorder
                .publish(Event {
                    type_: EventType::Normal,
                    reason: "DroppingDatabase".into(),
                    note: Some(format!("Dropping database `{database_name}`")),
                    action: "Dropping Database".into(),
                    secondary: None,
                })
                .await?;
            dbc.drop_database(database_name).await?;
            let (owner, _) = self.get_credentials(&ctx.client).await?;
            recorder
                .publish(Event {
                    type_: EventType::Normal,
                    reason: "DroppingUser".into(),
                    note: Some(format!("Dropping user `{owner}`")),
                    action: "Dropping User".into(),
                    secondary: None,
                })
                .await?;
            dbc.drop_user(owner.as_ref()).await?;
        }
        Ok(Action::await_change())
    }
}

impl DatabaseServer {
    async fn get_credentials(&self, client: &Client) -> Result<(String, String), Error> {
        let namespace = self
            .namespace()
            .ok_or(Error::MissingNamespace(self.name_any()))?;
        let secret_name = self.spec.superuser_secret.as_ref();
        let secret = Api::<Secret>::namespaced(client.clone(), &namespace)
            .get(secret_name)
            .await?;

        let username = String::from_utf8(
            secret
                .data
                .as_ref()
                .ok_or(Error::SecretMissingKey(
                    secret_name.to_owned(),
                    "username".to_owned(),
                ))?
                .get("username")
                .ok_or(Error::SecretMissingKey(
                    secret_name.to_owned(),
                    "username".to_owned(),
                ))?
                .0
                .clone(),
        )
        .map_err(|e| Error::SecretDidNotContainValidUTF8(secret_name.to_owned(), e.to_string()))?;

        let password = String::from_utf8(
            secret
                .data
                .as_ref()
                .ok_or(Error::SecretMissingKey(
                    secret_name.to_owned(),
                    "password".to_owned(),
                ))?
                .get("password")
                .ok_or(Error::SecretMissingKey(
                    secret_name.to_owned(),
                    "password".to_owned(),
                ))?
                .0
                .clone(),
        )
        .map_err(|e| Error::SecretDidNotContainValidUTF8(secret_name.to_owned(), e.to_string()))?;

        Ok((username, password))
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
    diagnostics: Arc<RwLock<Diagnostics>>,
}

impl State {
    pub fn diagnostics(&self) -> Result<Diagnostics> {
        Ok(self.diagnostics.read()?.clone())
    }
    pub fn to_context(&self, client: Client) -> Arc<Context> {
        Arc::new(Context {
            client,
            diagnostics: self.diagnostics.clone(),
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
