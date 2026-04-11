use totp_rs::{Rfc6238 as RfcTotp, Rfc6238Error, TOTP, TotpUrlError};

use crate::random::{GenerateRandomBytesError, fill_random};

pub mod recovery_code;

static TOTP_LENGTH: usize = 6;

#[derive(thiserror::Error, Debug)]
#[error("failed to build totp setup data")]
pub enum BuildTotpSetupDataError {
    TotpCreation(#[from] TotpCreationError),
    TotpSecretBase64QrCode(#[from] TotpSecretBase64QrCodeError),
}

#[derive(thiserror::Error, Debug)]
#[error("failed to create totp qr code - {0}")]
pub struct TotpSecretBase64QrCodeError(String);

#[derive(Debug)]
pub struct TotpSetupData {
    pub secret_base32: String,
    pub secret_base64_qr_code: String,
}

pub(crate) fn generate_totp_secret() -> Result<Vec<u8>, GenerateRandomBytesError> {
    let mut secret = vec![0u8; 20];
    fill_random(&mut secret)?;

    Ok(secret)
}

pub(crate) fn build_totp_setup_data(
    secret: &[u8],
    username: &str,
) -> Result<TotpSetupData, BuildTotpSetupDataError> {
    let totp = build_totp(secret, username)?;

    let secret_base32 = totp.get_secret_base32();
    let secret_base64_qr_code = totp.get_qr_base64().map_err(TotpSecretBase64QrCodeError)?;

    Ok(TotpSetupData {
        secret_base32,
        secret_base64_qr_code,
    })
}

#[derive(thiserror::Error, Debug)]
#[error("failed to create totp for user")]
pub enum TotpCreationError {
    SystemTime(#[from] std::time::SystemTimeError),
    Rfc6238(#[from] Rfc6238Error),
    TotpUrl(#[from] TotpUrlError),
}

pub(crate) fn build_totp(secret: &[u8], username: &str) -> Result<TOTP, TotpCreationError> {
    let totp = RfcTotp::new(
        TOTP_LENGTH,
        secret.to_vec(),
        None,
        build_account_name(username),
    )?;

    Ok(TOTP::from_rfc6238(totp)?)
}

fn build_account_name(user_name: &str) -> String {
    format!("BitSync - {user_name}")
}
