#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(clippy::todo)]
#![warn(clippy::panic)]
mod condition;
mod controller;
mod heritage;

use std::sync::RwLockReadGuard;

pub use crate::controller::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Finalizer Error: {0}")]
    FinalizerError(#[source] Box<kube::runtime::finalizer::Error<Error>>),

    #[error("Kube Error: {0}")]
    KubeError(#[source] Box<kube::Error>),

    #[error("Tokio Postgres Error: {0}")]
    TokioPostgresError(#[source] Box<tokio_postgres::Error>),

    #[error("Error acquring lock")]
    LockError(String),

    #[error("Do not specify both secret and config map on {0}/{1}")]
    DoNotSpecifyBothSecretAndConfigMap(String, String),

    #[error("Specify at least one secret or config map")]
    SpecifyAtLeastOneSecretOrConfigMap,

    #[error("Missing namespace on object {0}")]
    MissingNamespace(String),

    #[error("ConfigMap {0} does not have the specified key {1}")]
    ConfigMapMissingKey(String, String),

    #[error("Secret {0} does not have the specified key {1}")]
    SecretMissingKey(String, String),

    #[error("Secret {0} did not contain valid UTF-8: {1}")]
    SecretDidNotContainValidUTF8(String, String),

    #[error("failed to serialize heritage for database {1}: {0}")]
    FailedToSerializeHeritage(#[source] Box<serde_json::Error>, String),

    #[error("failed to deserialize heritage for database {1}: {0}")]
    FailedToDeserializeHeritage(#[source] Box<serde_json::Error>, String),

    #[error("Database {0} is missing comment {1}")]
    MissingHeritage(String, String),

    #[error("Database {0} failed validation. {1} has value {2}, expected {3}")]
    HeritageValidation(String, String, String, String),
}

impl From<kube::Error> for Error {
    fn from(e: kube::Error) -> Self {
        Self::KubeError(Box::new(e))
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(e: tokio_postgres::Error) -> Self {
        Self::TokioPostgresError(Box::new(e))
    }
}

impl From<std::sync::PoisonError<RwLockReadGuard<'_, Diagnostics>>> for Error {
    fn from(e: std::sync::PoisonError<RwLockReadGuard<Diagnostics>>) -> Self {
        Self::LockError(e.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
mod dbc;
