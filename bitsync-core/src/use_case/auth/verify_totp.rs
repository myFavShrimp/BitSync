use bitsync_database::entity::User;

use crate::{
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
    Jwt(#[from] crate::jwt::Error),
}

#[derive(thiserror::Error, Debug)]
#[error("user provided an invalid totp value")]
pub struct TotpInvalidError;

#[derive(thiserror::Error, Debug)]
#[error("totp is not set up")]
pub struct TotpNotSetUpError;

pub async fn verify_totp(
    user: &User,
    session_id: &uuid::Uuid,
    totp_value: &str,
    jwt_secret: &str,
) -> Result<String, VerifyTotpError> {
    let active_secret = user.active_totp_secret.as_ref().ok_or(TotpNotSetUpError)?;

    let totp = build_totp(active_secret, &user.username)?;

    if !totp.check_current(totp_value)? {
        Err(TotpInvalidError)?;
    }

    let jwt = JwtClaims {
        sub: *session_id,
        login_state: LoginState::Full,
    }
    .encode(jwt_secret)?;

    Ok(jwt)
}
