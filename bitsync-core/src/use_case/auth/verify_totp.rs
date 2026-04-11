use bitsync_database::entity::User;

use crate::{
    jwt::{JwtClaims, LoginState},
    totp::{TotpCreationError, build_totp_for_user},
};

#[derive(thiserror::Error, Debug)]
#[error("failed to verify totp")]
pub enum VerifyTotpError {
    TotpCreation(#[from] TotpCreationError),
    SystemTime(#[from] std::time::SystemTimeError),
    TotpInvalid(#[from] TotpInvalid),
    TotpNotSetUp(#[from] TotpNotSetUp),
    Jwt(#[from] crate::jwt::Error),
}

#[derive(thiserror::Error, Debug)]
#[error("user provided an invalid totp value")]
pub struct TotpInvalid;

#[derive(thiserror::Error, Debug)]
#[error("totp is not set up")]
pub struct TotpNotSetUp;

pub async fn verify_totp(
    user: &User,
    session_id: &uuid::Uuid,
    totp_value: &str,
    jwt_secret: &str,
) -> Result<String, VerifyTotpError> {
    if !user.is_totp_set_up {
        return Err(TotpNotSetUp)?;
    }

    let totp = build_totp_for_user(user)?;

    if !totp.check_current(totp_value)? {
        Err(TotpInvalid)?;
    }

    let jwt = JwtClaims {
        sub: *session_id,
        login_state: LoginState::Full,
    }
    .encode(jwt_secret)?;

    Ok(jwt)
}
