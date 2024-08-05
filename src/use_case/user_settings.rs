use std::sync::Arc;

use crate::{auth::AuthData, database::user::User, hash::hash_password, AppState};

#[derive(thiserror::Error, Debug)]
#[error("An unexpected error occurred")]
pub enum UserSettingsError {
    PasswordHash(#[from] argon2::password_hash::Error),
    Database(#[from] sqlx::Error),
}

pub async fn update_password(
    app_state: &Arc<AppState>,
    auth_data: &AuthData,
    new_password: &str,
) -> Result<User, UserSettingsError> {
    let hashed_password = hash_password(new_password)?;

    Ok(User::update_password(
        &app_state.postgres_pool,
        &auth_data.user.id,
        &hashed_password,
    )
    .await?)
}
