use std::path::PathBuf;

use crate::{
    database::user::User,
    handler::api::graphql::PublicContext,
    hash::hash_password,
    storage::{
        Storage, StorageError, StorageItemPath, StorageItemPathError, StorageKind, UserStorage,
    },
};

#[derive(thiserror::Error, Debug)]
pub enum RegistrationError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
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

pub async fn perform_registration<'context>(
    ctx: &async_graphql::Context<'context>,
    username: String,
    password: String,
) -> Result<User, RegistrationError> {
    let context = ctx
        .data::<PublicContext>()
        .map_err(RegistrationError::Context)?;

    if let Ok(_user) = User::find_by_username(&context.app_state.postgres_pool, &username).await {
        return Err(RegistrationError::UserExists);
    }

    let hashed_password = hash_password(&password)?;
    let user = User::create(
        &context.app_state.postgres_pool,
        &username,
        &hashed_password,
    )
    .await?;

    let user_storage = UserStorage {
        user: user.clone(),
        storage_root: context.app_state.config.fs_storage_root_dir.clone(),
    };
    let path = StorageItemPath::new(user_storage.clone(), PathBuf::from("/"))?;

    let storage = StorageKind::create(&context.app_state.config).await;

    storage.create_directory(&path).await?;

    Ok(user)
}
