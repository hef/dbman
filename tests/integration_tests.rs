use controller::State;
use kube::{
    api::{DeleteParams, PostParams},
    core::ObjectMeta,
    runtime::{conditions::is_deleted, wait::await_condition},
    Api, ResourceExt,
};
mod common;

use crate::common::{DatabaseServerHandle, ScopedNamespace};

#[tokio::test]
async fn test_basic() {
    env_logger::init();
    let client = common::get_kube_client().await;
    // setup cpng needs to come before install crds, as crds waits for cpng's crds to be ready
    common::setup_cnpg(&client).await;
    common::instal_crds(&client).await;
    let dbname = "my-db";
    let database_server = "my-db-cluster";

    let namespace = ScopedNamespace::new(client.clone(), "dbman-test".into()).await;
    let handle = DatabaseServerHandle::new(&client, namespace.name.clone()).await;

    let (dbc, conn) = handle.connect(&client).await;
    tokio::spawn(async move {
        conn.await.unwrap();
    });

    let result = dbc
        .query(
            "select 1 from pg_database where datname = $1::TEXT",
            &[&dbname],
        )
        .await
        .unwrap();

    assert_eq!(0, result.len());

    common::store_credentials_in_secret(
        &client,
        namespace.name.clone(),
        "my-db-credentials".into(),
        "my-username".into(),
        "my-password".into(),
    )
    .await;

    handle
        .create_database_server(&client, namespace.name.clone(), database_server.into())
        .await;

    let db = controller::Database {
        metadata: ObjectMeta {
            name: Some(dbname.into()),
            namespace: Some(namespace.name.clone()),
            ..Default::default()
        },
        spec: controller::DatabaseSpec {
            database_server_ref: controller::DatabaseServerRef {
                name: database_server.into(),
                namespace: Some(namespace.name.clone()),
            },
            database_name: dbname.into(),
            credentials_secret: "my-db-credentials".into(),
            prune: Some(true),
        },
        status: Some(controller::DatabaseStatus { conditions: vec![] }),
    };

    let db_api = Api::<controller::Database>::namespaced(client.clone(), &namespace.name);
    let db_object = db_api.create(&PostParams::default(), &db).await.unwrap();

    tokio::spawn(controller::run(State::default()));

    let db_ready = await_condition(db_api.clone(), dbname, common::is_database_ready());
    let _ = tokio::time::timeout(std::time::Duration::from_secs(300), db_ready) // todo: set back to 30
        .await
        .unwrap()
        .unwrap();

    let result = dbc
        .query(
            "select 1 from pg_database where datname = $1::TEXT",
            &[&dbname],
        )
        .await
        .unwrap();

    assert_eq!(1, result.len());

    // namespace cleanup doesn't handle missing finalizers yet.  just delete the object for now
    db_api
        .delete(dbname, &DeleteParams::default())
        .await
        .unwrap();
    let uid = db_object.uid().unwrap();
    let deleted = await_condition(db_api.clone(), dbname, is_deleted(&uid));
    let _ = tokio::time::timeout(std::time::Duration::from_secs(300), deleted)
        .await
        .unwrap()
        .expect("timed out deleting api");

    let _result = dbc
        .query(
            "select 1 from pg_database where datname = $1::TEXT",
            &[&dbname],
        )
        .await
        .unwrap();

    assert_eq!(0, result.len());
}
