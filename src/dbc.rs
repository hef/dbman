use log::info;
use postgres_protocol::escape::{escape_identifier, escape_literal};
use tokio_postgres::{Client, NoTls};

pub struct Dbc {
    client: Client,
    _join_handle: tokio::task::JoinHandle<()>,
}

impl Dbc {
    pub async fn new(config: &str) -> Result<Self, tokio_postgres::Error> {
        info!("Connecting to {}", config);
        let (client, connection) = tokio_postgres::connect(config, NoTls).await?;
        let _join_handle = tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Self {
            client,
            _join_handle,
        })
    }

    pub async fn does_user_exist(&self, user: &str) -> Result<bool, tokio_postgres::Error> {
        let result = self
            .client
            .query("select 1 from pg_roles where rolname = $1::TEXT", &[&user])
            .await?;
        Ok(!result.is_empty())
    }

    pub async fn create_user(&self, user: &str) -> Result<(), tokio_postgres::Error> {
        info!("Creating user {}", user);
        // prepared statements don't work with create user, so we need string interpolation
        self.client
            .execute(&format!("create user {}", escape_identifier(user)), &[])
            .await?;
        Ok(())
    }

    pub async fn drop_user(&self, user: &str) -> Result<(), tokio_postgres::Error> {
        info!("Dropping user {}", user);
        self.client
            .execute(&format!("drop user {}", escape_identifier(user)), &[])
            .await?;
        Ok(())
    }

    pub async fn update_password(
        &self,
        user: &str,
        password: &str,
    ) -> Result<(), tokio_postgres::Error> {
        info!("Updating password for user {}", user);
        self.client
            .execute(
                &format!(
                    "alter user {} with encrypted password {}",
                    escape_identifier(user),
                    escape_literal(password)
                ),
                &[],
            )
            .await?;
        Ok(())
    }

    pub async fn does_database_exist(&self, database: &str) -> Result<bool, tokio_postgres::Error> {
        let result = self
            .client
            .query(
                "select 1 from pg_database where datname = $1::TEXT",
                &[&database],
            )
            .await?;
        Ok(!result.is_empty())
    }

    pub async fn create_database(
        &self,
        owner: &str,
        database: &str,
    ) -> Result<(), tokio_postgres::Error> {
        info!("Creating database {}", database);
        self.client
            .execute(
                &format!(
                    "create database {} with owner = {}",
                    escape_identifier(database),
                    escape_identifier(owner)
                ),
                &[],
            )
            .await?;
        Ok(())
    }

    pub async fn drop_database(&self, database: &str) -> Result<(), tokio_postgres::Error> {
        info!("Dropping database {}", database);
        self.client
            .execute(
                &format!("drop database {}", escape_identifier(database)),
                &[],
            )
            .await?;
        Ok(())
    }

    pub async fn grant_all_privileges_on_database_to_user(
        &self,
        database: &str,
        user: &str,
    ) -> Result<(), tokio_postgres::Error> {
        info!(
            "Granting all privileges on database {} to user {}",
            database, user
        );
        self.client
            .execute(
                &format!(
                    "grant all privileges on database {} to {}",
                    escape_identifier(database),
                    escape_identifier(user)
                ),
                &[],
            )
            .await?;
        Ok(())
    }
}
