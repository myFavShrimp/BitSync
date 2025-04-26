use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::User,
    repository::{self, QueryError},
};

use crate::{
    hash::{hash_password, PasswordHashCreationError},
    totp::{build_totp_for_user, TotpCreationError},
};

#[derive(thiserror::Error, Debug)]
#[error("Failed to setup totp")]
pub enum RetrieveTotpSetupDataError {
    TotpCreation(#[from] TotpCreationError),
    TotpSecretBase64QrCode(#[from] TotpSecretBase64QrCodeError),
    SystemTime(#[from] std::time::SystemTimeError),
    TotpInvalid(#[from] TotpInvalid),
    ConnectionAquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
    PasswordHashCreation(#[from] PasswordHashCreationError),
}

#[derive(thiserror::Error, Debug)]
#[error("user provided an invalid totp value")]
pub struct TotpInvalid;

#[derive(thiserror::Error, Debug)]
#[error("Failed to create totp qr code - {0}")]
pub struct TotpSecretBase64QrCodeError(String);

pub struct TotpSetupResult {
    pub recovery_codes: Vec<String>,
}

const RECOVERY_CODE_COUNT: usize = 8;
const _: () = assert!(RECOVERY_CODE_COUNT % 4 == 0, "count must be divisible by 4");

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

            let mut recovery_codes = Vec::with_capacity(RECOVERY_CODE_COUNT);
            for _ in 0..(RECOVERY_CODE_COUNT / 4) {
                let codes = recovery_codes_for_user();

                let hashed_codes = [
                    hash_password(&codes[0])?,
                    hash_password(&codes[1])?,
                    hash_password(&codes[2])?,
                    hash_password(&codes[3])?,
                ];

                repository::totp_recovery_code::create(&mut *conn, user.id, &hashed_codes).await?;

                recovery_codes.push(codes[0].clone());
                recovery_codes.push(codes[1].clone());
                recovery_codes.push(codes[2].clone());
                recovery_codes.push(codes[3].clone());
            }

            Ok(TotpSetupResult { recovery_codes })
        }
        false => Err(TotpInvalid)?,
    }
}

// TODO: maybe make this more dynamically sized
fn recovery_codes_for_user() -> [String; 4] {
    let uuid_string = uuid::Uuid::new_v4().simple().to_string();
    let (first_code, rest) = uuid_string.split_at(8);
    let (second_code, rest) = rest.split_at(8);
    let (third_code, fourth_code) = rest.split_at(8);

    [
        first_code.to_string(),
        second_code.to_string(),
        third_code.to_string(),
        fourth_code.to_string(),
    ]
}
