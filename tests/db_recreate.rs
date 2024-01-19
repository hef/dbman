use controller::State;
use kube::{api::PostParams, core::ObjectMeta, Api};
use postgres_protocol::escape::escape_identifier;

use crate::common::{DatabaseServerHandle, ScopedNamespace};

mod common;
// https://github.com/hef/dbman/issues/75
// The issue is dbman failing to create a database that is deleted out of band.
// This test passes with no changes, not sure what is going on yet.
#[tokio::test]
async fn test_recreate_deleted_db() {
    let client: kube::Client = common::get_kube_client().await;
    // setup cpng needs to come before install crds, as crds waits for cpng's crds to be ready
    common::setup_cnpg(&client).await;
    common::instal_crds(&client).await;
    let state: State = State::default();
    let ctx = state.to_context(client.clone());
    let dbname = "my-db";
    let database_server = "my-db-cluster";

    let namespace = ScopedNamespace::new(client.clone(), "dbman-test-db-recreate".into()).await;
    let handle = DatabaseServerHandle::new(&client, namespace.name.clone()).await;

    let (dbc, conn) = handle.connect(&client).await;
    tokio::spawn(async move {
        conn.await.unwrap();
    });

    let exists = common::does_pgdatabase_exist(&dbc, &dbname.to_string()).await;
    assert!(!exists);

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
            credentials: Some(controller::Credentials {
                basic_auth_secret_ref: Some("my-db-credentials".into()),
                ..Default::default()
            }),
            prune: Some(true),
            ..Default::default()
        },
        status: Some(controller::DatabaseStatus { conditions: vec![] }),
    };

    let db_api = Api::<controller::Database>::namespaced(client.clone(), &namespace.name);
    let _db_object = db_api.create(&PostParams::default(), &db).await.unwrap();

    let db = db_api.get(dbname).await.expect("database cr exists");
    let z = db.z_reconcile(ctx.to_owned()).await;
    assert!(z.is_ok());

    let exists = common::does_pgdatabase_exist(&dbc, &dbname.to_string()).await;
    assert!(exists);

    // delete the database OOB
    dbc.execute(&format!("drop database {}", escape_identifier(dbname)), &[])
        .await
        .expect("dropping database manually");

    let exists = common::does_pgdatabase_exist(&dbc, &dbname.to_string()).await;
    assert!(!exists);

    let db = db_api.get(dbname).await.expect("database cr exists");
    let z = db.z_reconcile(ctx).await;
    assert!(z.is_ok());

    let exists = common::does_pgdatabase_exist(&dbc, &dbname.to_string()).await;
    assert!(exists);
}
