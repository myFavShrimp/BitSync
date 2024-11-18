use bitsync_database::{
    database::{Database, TransactionBeginError},
    entity::User,
    repository,
};

use crate::hash::{hash_password, PasswordHashCreationError};

#[derive(thiserror::Error, Debug)]
#[error("An unexpected error occurred")]
pub enum UpdateUserPasswordError {
    PasswordHash(#[from] PasswordHashCreationError),
    Database(#[from] repository::QueryError),
    TransactionBegin(#[from] TransactionBeginError),
}

pub async fn update_user_password(
    database: &Database,
    user: &User,
    new_password: &str,
) -> Result<User, UpdateUserPasswordError> {
    let hashed_password = hash_password(new_password)?;

    let mut transaction = database.begin_transaction().await?;

    Ok(repository::user::update_password(&mut *transaction, &user.id, &hashed_password).await?)
}
