use std::sync::Arc;

use crate::{auth::AuthData, database::user::User, jwt::JwtClaims, AppState};

#[derive(Debug, Clone, thiserror::Error)]
#[error("The provided auth token is invalid")]
pub struct AuthTokenInvalidError;

pub async fn decode_auth_token(
    app_state: Arc<AppState>,
    token: &str,
) -> Result<AuthData, AuthTokenInvalidError> {
    match JwtClaims::decode_and_validate(token, &app_state.config.jwt_secret) {
        Ok(claims) => match User::find_by_id(&app_state.postgres_pool, &claims.sub).await {
            Ok(user) => Ok(AuthData { claims, user }),
            Err(_) => Err(AuthTokenInvalidError),
        },
        Err(_) => Err(AuthTokenInvalidError),
    }
}
