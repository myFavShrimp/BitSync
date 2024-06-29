use std::sync::Arc;

use crate::jwt::JwtClaims;

use crate::{database::user::User, hash::verify_password_hash, AppState};

#[derive(thiserror::Error, Debug)]
#[error("Username or password wrong")]
pub enum LoginError {
    PasswordHash(#[from] argon2::password_hash::Error),
    Database(#[from] sqlx::Error),
    Jwt(#[from] jsonwebtoken::errors::Error),
}

pub async fn perform_login(
    app_state: &Arc<AppState>,
    username: String,
    password: String,
) -> Result<String, LoginError> {
    let user = User::find_by_username(&app_state.postgres_pool, &username).await?;

    verify_password_hash(&user.password, &password)?;

    let jwt_expiration =
        time::OffsetDateTime::now_utc().unix_timestamp() + app_state.config.jwt_expiration_seconds;

    Ok(JwtClaims {
        sub: user.id,
        exp: jwt_expiration,
    }
    .encode(&app_state.config.jwt_secret)?)
}
