use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::User,
    repository::{self, QueryError},
};

use crate::totp::{build_totp_for_user, TotpCreationError};

#[derive(thiserror::Error, Debug)]
#[error("Failed to setup totp")]
pub enum RetrieveTotpSetupDataError {
    TotpCreation(#[from] TotpCreationError),
    TotpSecretBase64QrCode(#[from] TotpSecretBase64QrCodeError),
    SystemTime(#[from] std::time::SystemTimeError),
    TotpInvalid(#[from] TotpInvalid),
    ConnectionAquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
}

#[derive(thiserror::Error, Debug)]
#[error("user provided an invalid totp value")]
pub struct TotpInvalid;

#[derive(thiserror::Error, Debug)]
#[error("Failed to create totp qr code - {0}")]
pub struct TotpSecretBase64QrCodeError(String);

pub struct TotpSetupResult {}

pub async fn setup_totp(
    database: &Database,
    user: &User,
    totp_value: &str,
) -> Result<TotpSetupResult, RetrieveTotpSetupDataError> {
    let totp = build_totp_for_user(user)?;

    match totp.check_current(totp_value)? {
        true => {
            let mut conn = database.acquire_connection().await?;
            repository::user::set_totp_setup_state(&mut *conn, &user.id, true).await?;

            todo!("setup totp recovery phrases")
        }
        false => Err(TotpInvalid)?,
    }

    Ok(TotpSetupResult {})
}

// fn recovery_codes_for_user()
