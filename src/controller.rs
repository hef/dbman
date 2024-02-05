use crate::{
    condition::{Reason, Status, Type},
    dbc::Dbc,
    v1alpha2::DatabaseServer,
    v1alpha3, Error, Result,
};
use async_recursion::async_recursion;
use k8s_openapi::{
    apimachinery::pkg::apis::meta::v1::{Condition, Time},
    chrono::{DateTime, Utc},
};
use log::{error, warn};
use std::{
    collections::{hash_map::DefaultHasher, HashMap}, hash::Hasher, sync::{Arc, RwLock}, time::Duration
};


use crate::heritage::Heritage;
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
    Api, Client, Resource, ResourceExt,
};
use log::info;
use serde::Serialize;

pub static DATABASE_FINALIZER: &str = "databases.hef.sh/finalizer";


#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ModificationEntry {
    txid: i32,
    hash: u64,
}

#[derive(Clone)]
pub struct Context {
    client: Client,
    diagnostics: Arc<RwLock<Diagnostics>>,
    modification_cache: Arc<RwLock<HashMap<String, ModificationEntry>>>,
}

async fn reconcile(db: Arc<v1alpha3::Database>, ctx: Arc<Context>) -> Result<Action> {
    let ns = db
        .namespace()
        .ok_or(Error::MissingNamespace(db.name_any()))?;
    let dbs: Api<v1alpha3::Database> = Api::namespaced(ctx.client.clone(), &ns);

    info!("Reconciling Database \"{}\" in {}", db.name_any(), ns);
    finalizer(&dbs, DATABASE_FINALIZER, db, |event| async {
        match event {
            Finalizer::Apply(db) => match db.reconcile(ctx.clone()).await {
                Ok(action) => Ok(action),
                Err(e) => {
                    let client = ctx.client.clone();
                    let recorder = ctx.diagnostics.read()?.recorder(client.clone(), &db);
                    db.set_condition(
                        &client,
                        Type::Ready,
                        Status::False,
                        Reason::ReconcileError,
                        &e.to_string(),
                    )
                    .await
                    .ok();
                    recorder
                        .publish(Event {
                            type_: EventType::Warning,
                            reason: "ReconcilingError".into(),
                            note: Some(e.to_string()),
                            action: "Reconciling".into(),
                            secondary: None,
                        })
                        .await
                        .ok();
                    error!("error reconciling: {}", e);
                    Err(e)
                }
            },
            Finalizer::Cleanup(db) => match db.cleanup(ctx.clone()).await {
                Ok(action) => {
                    if let Some(uid) = db.metadata.uid.as_ref() {
                        ctx.to_owned().modification_cache.write()?.remove(uid);
                    }
                    Ok(action)
                },
                Err(e) => {
                    let client = ctx.client.clone();
                    let recorder = ctx.diagnostics.read()?.recorder(client.clone(), &db);
                    db.set_condition(
                        &client,
                        Type::Finalized,
                        Status::False,
                        Reason::FinalizeError,
                        &e.to_string(),
                    )
                    .await
                    .ok();
                    recorder
                        .publish(Event {
                            type_: EventType::Warning,
                            reason: "FinalizeError".into(),
                            note: Some(e.to_string()),
                            action: "Finalizing".into(),
                            secondary: None,
                        })
                        .await
                        .ok();
                    error!("error finalizing: {}", e);
                    Err(e)
                }
            },
        }
    })
    .await
    .map_err(|e| Error::FinalizerError(Box::new(e)))
}

fn error_policy(_database: Arc<v1alpha3::Database>, error: &Error, _ctx: Arc<Context>) -> Action {
    warn!("reconcile failed: {:?}", error);
    Action::requeue(Duration::from_secs(5 * 60))
}

impl v1alpha3::Database {
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


        // todo: this hasher is not required or used
        let hasher = &mut DefaultHasher::new();

        let (superuser_name, superuser_password) = dbs.get_credentials(client, hasher).await?;
        let dbc = Dbc::new(&dbs.spec.conn_string, &superuser_name, &superuser_password)
            .await
            .map_err(|e| {
                if let Some(db_error) = e.as_db_error() {
                    if db_error.code() == &tokio_postgres::error::SqlState::UNDEFINED_DATABASE {
                        return Error::DatabaseServerNeedsToSpecifyAnExistingDbName(
                            database_server_namespace,
                            self.spec.database_server_ref.name.clone(),
                        );
                    }
                }
                e.into()
            })?;
        Ok(dbc)
    }

    async fn get_credentials(&self, client: &Client) -> Result<Option<(String, String, u64)>, Error> {

        let hasher = &mut DefaultHasher::new();
        if let (Some(uid), Some(resource_version)) = (self.metadata.uid.as_ref(), self.metadata.resource_version.as_ref()) {
            hasher.write(uid.as_bytes());
            hasher.write(resource_version.as_bytes());
        } // todo raise an error if uid or resource_version is missing

        if let Some(credentials) = &self.spec.credentials {
            let namespace = self
                .namespace()
                .ok_or(Error::MissingNamespace(self.name_any()))?;

            let (username, password) = credentials
                .get_credentials(client, namespace.as_str(), hasher)
                .await?;
            let hash = hasher.finish(); // todo: 64 bit hash is not enough
            Ok(Some((username, password, hash)))
        } else {
            Ok(None)
        }
    }

    #[async_recursion] // todo: it would be nice to not copy visited, like some immutable array type or something
    async fn get_owner(
        &self,
        client: &Client,
        visited: Vec<String>,
    ) -> Result<Option<String>, Error> {

        if let Some(other_db) = &self.spec.owner_ref {
            let namespace = self
                .namespace()
                .ok_or(Error::MissingNamespace(self.name_any()))?;
            let api: Api<v1alpha3::Database> = Api::namespaced(client.clone(), &namespace);
            let other_db: v1alpha3::Database = api.get(other_db).await?;

            let mut visited = visited.to_vec(); // Change the type of visited to Vec<String>

            if visited.contains(&other_db.name_any()) {
                return Err(Error::CircularDependency(visited.to_vec()));
            }

            visited.push(other_db.name_any());

            return other_db.get_owner(client, visited).await; // Pass visited as a reference
        }

        if let Some(credentials) = &self.spec.credentials {
            let namespace = self
                .namespace()
                .ok_or(Error::MissingNamespace(self.name_any()))?;

            let hasher = &mut DefaultHasher::new(); // todo: don't need this hasher
            let (owner, _) = credentials
                // todo: don't get password, it might be an extra api lookup
                .get_credentials(client, namespace.as_str(), hasher)
                .await?;
            Ok(Some(owner))
        } else {
            Ok(None)
        }
    }

    #[cfg(feature = "test-utils")]
    pub async fn z_reconcile(&self, ctx: Arc<Context>) -> Result<Action> {
        self.reconcile(ctx).await
    }

    async fn reconcile(&self, ctx: Arc<Context>) -> Result<Action> {
        let client = ctx.client.clone();
        let dbc = self.dbc(&client).await?;
        let recorder = ctx.diagnostics.read()?.recorder(client.clone(), self);
        let heritage = Heritage::builder().resource(self).build();

        let uid = if let Some(uid) =self.metadata.uid.as_ref(){
            uid
        } else {
            return Err(Error::MissingUidOrResourceVersion(self.name_any()));
        };
        

        let mut owner: Option<String> = None;

        if let Some((username, password, hash)) = self.get_credentials(&client).await? {
            owner = Some(username.clone());
            if !dbc.does_user_exist(&username).await? {
                recorder
                    .publish(Event {
                        type_: EventType::Normal,
                        reason: "CreatingUser".into(),
                        note: Some(format!("Creating user `{username}`")),
                        action: "Creating User".into(),
                        secondary: None,
                    })
                    .await?;
                dbc.create_user(&username).await?;
                dbc.apply_heritage_to_role(&username, &heritage).await?;
                recorder
                    .publish(Event {
                        type_: EventType::Normal,
                        reason: "UpdatingPassword".into(),
                        note: Some(format!("Updating password for `{username}`")),
                        action: "Updating Password".into(),
                        secondary: None,
                    })
                    .await?;
                dbc.update_password(&username, &password).await?;
                let txid = dbc.get_role_txid(&username).await?; // todo: role get txid into update_password
                let mut cache = ctx.modification_cache.write()?;
                cache.insert(uid.to_owned(), ModificationEntry { txid, hash });
            } else {
                dbc.validate_heritage_on_role(&username, &heritage).await?;
                let txid = dbc.get_role_txid(&username).await?;
                
                let z = {
                    let cache = ctx.modification_cache.read()?;
                    cache.get(uid).cloned()
                };
                if let Some(e) = z.or(Default::default()) {
                    if (e.txid != txid && e.hash != hash) || (e.txid == 0 && e.hash == 0) { // todo: there is probably is isDefault check somewhere
                        recorder
                            .publish(Event {
                                type_: EventType::Normal,
                                reason: "UpdatingPassword".into(),
                                note: Some(format!("Updating password for `{username}`")),
                                action: "Updating Password".into(),
                                secondary: None,
                            })
                            .await?;
                        dbc.update_password(&username, &password).await?;
                        let mut cache = ctx.modification_cache.write()?;
                        cache.insert(uid.to_owned(), ModificationEntry { txid, hash });
                    }
                }
            }
        }

        if owner.is_none() {
            owner = self.get_owner(&client, Vec::new()).await?;
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

            if let Some(owner) = owner.clone() {
                dbc.create_database_with_owner(database_name, &owner)
                    .await?;
            } else {
                dbc.create_database(database_name).await?;
            }
            dbc.apply_heritage_to_database(database_name, &heritage)
            .await?;

            if let Some(owner) = owner {
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
                dbc.validate_heritage_on_database(database_name, &heritage)
                    .await?;

                dbc.grant_all_privileges_on_database_to_user(database_name, &owner)
                    .await?;
            }
        } else if let Some(owner) = owner.clone() {
            dbc.validate_heritage_on_database(database_name, &heritage).await?;
            let actual_owner = dbc.get_database_owner(database_name).await?;
            if actual_owner != owner {
                recorder
                    .publish(Event {
                        type_: EventType::Normal,
                        reason: "UpdatingOwner".into(),
                        note: Some(format!(
                            "Updating owner of database `{database_name}` to `{owner}`"
                        )),
                        action: "Updating Owner".into(),
                        secondary: None,
                    })
                    .await?;
                dbc.set_database_owner(database_name, &owner).await?;
            }
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
        let api = Api::<v1alpha3::Database>::namespaced(
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
        let heritage = Heritage::builder().resource(self).build();
        let database_name = &self.spec.database_name;
        dbc.validate_heritage_on_database(database_name, &heritage)
            .await?;
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
            if let Some((owner, _, _)) = self.get_credentials(&ctx.client).await? {
                recorder
                    .publish(Event {
                        type_: EventType::Normal,
                        reason: "DroppingUser".into(),
                        note: Some(format!("Dropping user `{owner}`")),
                        action: "Dropping User".into(),
                        secondary: None,
                    })
                    .await?;
                dbc.validate_heritage_on_role(owner.as_ref(), &heritage)
                    .await?;
                dbc.drop_user(owner.as_ref()).await?;
            }
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
    fn recorder(&self, client: Client, db: &v1alpha3::Database) -> Recorder {
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
            modification_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

}

pub async fn run(state: State) -> Result<(), Error> {
    let client = Client::try_default().await?;
    let databases = Api::<v1alpha3::Database>::all(client.clone());

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
