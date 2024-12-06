use bitsync_database::{
    database::{Database, TransactionBeginError},
    entity::User,
    repository,
};

use crate::hash::{hash_password, PasswordHashCreationError};

#[derive(thiserror::Error, Debug)]
#[error("")]
pub enum ReadUserSettingsError {
    PasswordHash(#[from] PasswordHashCreationError),
    Database(#[from] repository::QueryError),
    TransactionBegin(#[from] TransactionBeginError),
}
