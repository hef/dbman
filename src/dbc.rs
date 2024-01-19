use log::info;
use postgres_protocol::escape::{escape_identifier, escape_literal};
use tokio_postgres::{Client, NoTls};

use crate::{heritage::Heritage, Error};

pub(crate) struct Dbc {
    client: Client,
    _join_handle: tokio::task::JoinHandle<()>,
}

impl Dbc {
    pub async fn new(
        config: &str,
        username: &str,
        password: &str,
    ) -> Result<Self, tokio_postgres::Error> {
        info!("Connecting to {}", config);
        let full_config = format!("{} user={} password={}", config, username, password);
        let (client, connection) = tokio_postgres::connect(&full_config, NoTls).await?;
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

    pub async fn apply_heritage_to_database(
        &self,
        database_name: &str,
        heritage: &Heritage,
    ) -> Result<(), Error> {
        let heritage_text = serde_json::to_string(heritage)
            .map_err(|e| Error::FailedToSerializeHeritage(Box::new(e), database_name.into()))?;
        self.client
            .execute(
                &format!(
                    "COMMENT ON DATABASE {} IS {}",
                    escape_identifier(database_name),
                    escape_literal(&heritage_text)
                ),
                &[],
            )
            .await?;
        Ok(())
    }

    pub async fn apply_heritage_to_role(
        &self,
        role_name: &str,
        heritage: &Heritage,
    ) -> Result<(), Error> {
        let heritage_text = serde_json::to_string(heritage)
            .map_err(|e| Error::FailedToSerializeHeritage(Box::new(e), role_name.into()))?;
        //let heritage_text = "sup";
        self.client
            .execute(
                &format!(
                    "COMMENT ON ROLE {} IS {}",
                    escape_identifier(role_name),
                    escape_literal(&heritage_text)
                ),
                &[],
            )
            .await?;
        Ok(())
    }

    pub async fn validate_heritage_on_database(
        &self,
        database_name: &str,
        heritage: &Heritage,
    ) -> Result<(), Error> {
        let result = self.client.query(
            "select description from pg_shdescription join pg_database on objoid = pg_database.oid where datname = $1::TEXT",
         &[&database_name]).await?;
        //let result = self.client.query(&format!("select description from pg_shdescription join pg_database on objoid = pg_database.oid where datname = {}", escape_identifier(database)), &[]).await?;
        if result.len() != 1 {
            return Err(Error::MissingHeritage(
                database_name.into(),
                serde_json::to_string(heritage).map_err(|e| {
                    Error::FailedToSerializeHeritage(Box::new(e), database_name.into())
                })?,
            ));
        }
        let description: String = result[0].get(0);
        heritage
            .validate(database_name, &description)
            .map_err(|e| match e {
                Error::HeritageValidation(database_name, field, value, expected) => {
                    Error::DatabaseHeritageValidation(database_name, field, value, expected)
                }
                _ => e,
            })?;
        Ok(())
    }

    pub async fn validate_heritage_on_role(
        &self,
        role_name: &str,
        heritage: &Heritage,
    ) -> Result<(), Error> {
        let result = self.client.query(
            "select description from pg_shdescription join pg_roles on objoid = pg_roles.oid where rolname = $1::TEXT",
            &[&role_name]).await?;
        //let result = self.client.query(&format!("select description from pg_shdescription join pg_database on objoid = pg_database.oid where datname = {}", escape_identifier(database)), &[]).await?;
        if result.len() != 1 {
            return Err(Error::MissingHeritage(
                role_name.into(),
                serde_json::to_string(heritage)
                    .map_err(|e| Error::FailedToSerializeHeritage(Box::new(e), role_name.into()))?,
            ));
        }
        let description: String = result[0].get(0);
        heritage
            .validate(role_name, &description)
            .map_err(|e| match e {
                Error::HeritageValidation(role_name, field, value, expected) => {
                    Error::RoleHeritageValidation(role_name, field, value, expected)
                }
                _ => e,
            })?;
        Ok(())
    }
}
