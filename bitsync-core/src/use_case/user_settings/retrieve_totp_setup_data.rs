use bitsync_database::entity::User;

use crate::totp::{build_totp_for_user, TotpCreationError};

#[derive(thiserror::Error, Debug)]
#[error("Failed to create totp setup data")]
pub enum RetrieveTotpSetupDataError {
    TotpCreation(#[from] TotpCreationError),
    TotpSecretBase64QrCode(#[from] TotpSecretBase64QrCodeError),
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to create totp qr code - {0}")]
pub struct TotpSecretBase64QrCodeError(String);

pub struct TotpSetupData {
    pub secret_base32: String,
    pub secret_base64_qr_code: String,
}

pub async fn retrieve_totp_setup_data(
    user: &User,
) -> Result<TotpSetupData, RetrieveTotpSetupDataError> {
    let totp = build_totp_for_user(user)?;

    let secret_base32 = totp.get_secret_base32();
    let secret_base64_qr_code = totp.get_qr_base64().map_err(TotpSecretBase64QrCodeError)?;

    Ok(TotpSetupData {
        secret_base32,
        secret_base64_qr_code,
    })
}
