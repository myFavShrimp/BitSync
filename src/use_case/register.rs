use std::{path::PathBuf, sync::Arc};

use crate::{
    database::user::User,
    hash::hash_password,
    storage::{Storage, StorageError, StorageItemPath, StorageItemPathError, UserStorage},
    AppState,
};

#[derive(thiserror::Error, Debug)]
pub enum RegistrationError {
    #[error("An unexpected error occurred")]
    PasswordHash(#[from] argon2::password_hash::Error),
    #[error("An unexpected error occurred")]
    Database(#[from] sqlx::Error),
    #[error("The username already exists")]
    UserExists,
    #[error(transparent)]
    StorageItemPathCreation(#[from] StorageItemPathError),
    #[error("Error handling the create operation")]
    Storage(#[from] StorageError),
}

pub async fn perform_registration(
    app_state: &Arc<AppState>,
    username: &str,
    password: &str,
) -> Result<User, RegistrationError> {
    if let Ok(_user) = User::find_by_username(&app_state.postgres_pool, username).await {
        return Err(RegistrationError::UserExists);
    }

    let hashed_password = hash_password(password)?;
    let user = User::create(&app_state.postgres_pool, username, &hashed_password).await?;

    let user_storage = UserStorage {
        user: user.clone(),
        storage_root: app_state.config.fs_storage_root_dir.clone(),
    };
    let path = StorageItemPath::new(user_storage.clone(), PathBuf::from("/"))?;

    let storage = Storage::create();

    storage.create_directory(&path).await?;

    Ok(user)
}
