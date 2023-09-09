#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(clippy::todo)]
#![warn(clippy::panic)]
pub mod controller;

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
