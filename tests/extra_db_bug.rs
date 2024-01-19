use controller::State;
use kube::{api::PostParams, core::ObjectMeta, runtime::wait::await_condition, Api, Client};

use crate::common::{DatabaseServerHandle, ScopedNamespace};

mod common;

async fn create_database_server(
    handle: &DatabaseServerHandle,
    client: &Client,
    namespace: String,
    name: String,
    secret_name: String,
) {
    let port = handle.get_port();
    let dbs = controller::DatabaseServer {
        metadata: ObjectMeta {
            name: Some(name),
            namespace: Some(namespace.clone()),
            ..Default::default()
        },
        spec: controller::DatabaseServerSpec {
            conn_string: format!("host=localhost port={port}"),
            credentials: controller::Credentials {
                basic_auth_secret_ref: Some(secret_name),
                ..Default::default()
            },
        },
    };

    let dbs_api = Api::<controller::DatabaseServer>::namespaced(client.clone(), &namespace);
    dbs_api.create(&PostParams::default(), &dbs).await.unwrap();
}

// postgres expects a database name with the same name as the role to exist.
#[tokio::test]
async fn test_dbman_expects_extra_db() {
    let client: kube::Client = common::get_kube_client().await;
    // setup cpng needs to come before install crds, as crds waits for cpng's crds to be ready
    common::setup_cnpg(&client).await;
    common::instal_crds(&client).await;
    let dbname = "my-db-bugtest";
    let database_server = "my-db-cluster";

    let namespace = ScopedNamespace::new(client.clone(), "dbman-test-extra-db-bug".into()).await;
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

    common::store_credentials_in_secret(
        &client,
        namespace.name.clone(),
        "my-superuser-db-credentials".into(),
        "my_superuser".into(),
        "my-superpassword".into(),
    )
    .await;

    let _result = dbc
        .execute(
            "CREATE ROLE my_superuser SUPERUSER LOGIN PASSWORD 'my-superpassword'",
            &[],
        )
        .await
        .expect("creating custom superuser failed");
    //assert_eq!(1, result);

    create_database_server(
        &handle,
        &client,
        namespace.name.clone(),
        database_server.into(),
        "my-superuser-db-credentials".into(),
    )
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
    let db_object = db_api.create(&PostParams::default(), &db).await.unwrap();

    tokio::spawn(controller::run(State::default()));

    let db_ready = await_condition(db_api.clone(), dbname, common::is_database_ready());

    // todo: turn this back on when I figure out how to handle expected errors in tests
    /*
    let _ = tokio::time::timeout(std::time::Duration::from_secs(30), db_ready) // todo: set back to 30
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

    delete_db_object(&db_api, &db_object).await;
    */
    //let result = dbc.execute("DELETE ROLE my-superuser", &[]).await.expect("creating custom superuser failed");
}
