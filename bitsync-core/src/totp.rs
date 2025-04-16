use bitsync_database::entity::User;
use totp_rs::{
    qrcodegen_image::image::EncodableLayout, Rfc6238 as RfcTotp, Rfc6238Error, TotpUrlError, TOTP,
};

static TOTP_LENGTH: usize = 6;

#[derive(thiserror::Error, Debug)]
#[error("failed to create totp for user")]
pub enum TotpCreationError {
    SystemTime(#[from] std::time::SystemTimeError),
    Rfc6238(#[from] Rfc6238Error),
    TotpUrl(#[from] TotpUrlError),
}

pub(crate) fn build_totp_for_user(user: &User) -> Result<TOTP, TotpCreationError> {
    let totp = RfcTotp::new(
        TOTP_LENGTH,
        user.totp_secret.as_bytes().to_vec(),
        None,
        build_account_name(&user.username),
    )?;

    Ok(TOTP::from_rfc6238(totp)?)
}

fn build_account_name(user_name: &str) -> String {
    format!("BitSync - {user_name}")
}
