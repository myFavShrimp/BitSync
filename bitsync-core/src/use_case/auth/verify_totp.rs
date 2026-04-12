use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::User,
    repository::{self, QueryError},
};

use crate::{
    hash::verify_password_hash,
    jwt::{JwtClaims, LoginState},
    totp::{TotpCreationError, build_totp},
};

#[derive(thiserror::Error, Debug)]
#[error("failed to verify totp")]
pub enum VerifyTotpError {
    TotpCreation(#[from] TotpCreationError),
    SystemTime(#[from] std::time::SystemTimeError),
    TotpInvalid(#[from] TotpInvalidError),
    TotpNotSetUp(#[from] TotpNotSetUpError),
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
    Jwt(#[from] crate::jwt::Error),
}

#[derive(thiserror::Error, Debug)]
#[error("the entered totp code is invalid")]
pub struct TotpInvalidError;

#[derive(thiserror::Error, Debug)]
#[error("totp is not set up")]
pub struct TotpNotSetUpError;

pub async fn verify_totp(
    database: &Database,
    user: &User,
    session_id: &uuid::Uuid,
    totp_value: &str,
    jwt_secret: &str,
) -> Result<String, VerifyTotpError> {
    let active_secret = user.active_totp_secret.as_ref().ok_or(TotpNotSetUpError)?;

    let totp = build_totp(active_secret, &user.username)?;

    let is_valid_totp = totp.check_current(totp_value)?;

    if !is_valid_totp {
        let mut connection = database.acquire_connection().await?;

        let stored_codes =
            repository::totp_recovery_code::find_by_user_id(&mut *connection, user.id).await?;

        let matched_hash = stored_codes
            .into_iter()
            .find(|recovery_code| verify_password_hash(&recovery_code.code, totp_value).is_ok())
            .map(|recovery_code| recovery_code.code)
            .ok_or(TotpInvalidError)?;

        repository::totp_recovery_code::delete(&mut *connection, &user.id, &matched_hash).await?;
    }

    let jwt = JwtClaims {
        sub: *session_id,
        login_state: LoginState::Full,
    }
    .encode(jwt_secret)?;

    Ok(jwt)
}
