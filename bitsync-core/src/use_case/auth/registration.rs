use std::path::PathBuf;

use bitsync_database::{
    database::{Database, TransactionBeginError},
    entity::User,
    repository,
};
use bitsync_storage::{
    operation::write::{ensure_user_storage_exists, EnsureUserStorageExistsError},
    user_storage::UserStorage,
};

use crate::hash::{hash_password, PasswordHashCreationError};

#[derive(thiserror::Error, Debug)]
#[error("user registration failed")]
pub enum RegistrationError {
    PasswordHash(#[from] PasswordHashCreationError),
    Database(#[from] repository::QueryError),
    TransactionBegin(#[from] TransactionBeginError),
    UserExists,
    EnsureUserStorageExists(#[from] EnsureUserStorageExistsError),
}

pub async fn perform_registration(
    database: &Database,
    storage_root_dir: &PathBuf,
    username: &str,
    password: &str,
) -> Result<User, RegistrationError> {
    let mut transaction = database.begin_transaction().await?;

    if let Ok(_user) = repository::user::find_by_username(&mut *transaction, username).await {
        return Err(RegistrationError::UserExists);
    }

    let hashed_password = hash_password(password)?;
    let user = repository::user::create(&mut *transaction, username, &hashed_password).await?;

    let user_storage = UserStorage {
        user_id: user.id,
        storage_root: storage_root_dir.clone(),
    };

    ensure_user_storage_exists(&user_storage).await?;

    Ok(user)
}
