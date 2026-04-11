use bitsync_database::{
    database::{Database, TransactionBeginError, transaction::TransactionCommitError},
    entity::User,
    repository::{self, QueryError},
};

use crate::{
    hash::{PasswordHashCreationError, hash_password},
    totp::recovery_code::{GenerateRecoveryCodeError, generate_recovery_codes_batch},
    totp::{
        BuildTotpSetupDataError, TotpCreationError, TotpSetupData, build_totp,
        build_totp_setup_data,
    },
};

#[derive(thiserror::Error, Debug)]
#[error("failed to reset totp")]
pub enum ResetTotpError {
    TotpCreation(#[from] TotpCreationError),
    SystemTime(#[from] std::time::SystemTimeError),
    TransactionBegin(#[from] TransactionBeginError),
    TransactionCommit(#[from] TransactionCommitError),
    Query(#[from] QueryError),
    PasswordHashCreation(#[from] PasswordHashCreationError),
    NoTotpResetInProgress(#[from] NoTotpResetInProgressError),
    InvalidTotpCode(#[from] InvalidTotpCodeError),
    GenerateRecoveryCode(#[from] GenerateRecoveryCodeError),
    BuildTotpSetupData(#[from] BuildTotpSetupDataError),
}

#[derive(thiserror::Error, Debug)]
#[error("no totp reset is in progress")]
pub struct NoTotpResetInProgressError;

#[derive(thiserror::Error, Debug)]
#[error("the entered totp code is invalid")]
pub struct InvalidTotpCodeError {
    pub setup_data: TotpSetupData,
}

pub struct ResetTotpResult {
    pub recovery_codes: Vec<String>,
}

const RECOVERY_CODE_COUNT: usize = 8;
const _: () = assert!(
    RECOVERY_CODE_COUNT.is_multiple_of(4),
    "count must be divisible by 4"
);

pub async fn reset_totp(
    database: &Database,
    user: &User,
    totp_value: &str,
) -> Result<ResetTotpResult, ResetTotpError> {
    let dangling_secret = user
        .dangling_totp_secret
        .as_ref()
        .ok_or(NoTotpResetInProgressError)?;

    let totp = build_totp(dangling_secret, &user.username)?;

    if !totp.check_current(totp_value)? {
        let setup_data = build_totp_setup_data(dangling_secret, &user.username)?;

        Err(InvalidTotpCodeError { setup_data })?;
    }

    let mut transaction = database.begin_transaction().await?;

    repository::user::activate_dangling_totp_secret(&mut *transaction, &user.id).await?;
    repository::totp_recovery_code::delete_all_for_user(&mut *transaction, &user.id).await?;

    let mut recovery_codes = Vec::with_capacity(RECOVERY_CODE_COUNT);
    for _ in 0..(RECOVERY_CODE_COUNT / 4) {
        let codes = generate_recovery_codes_batch()?;

        let hashed_codes = [
            hash_password(&codes[0])?,
            hash_password(&codes[1])?,
            hash_password(&codes[2])?,
            hash_password(&codes[3])?,
        ];

        repository::totp_recovery_code::create(&mut *transaction, user.id, &hashed_codes).await?;

        recovery_codes.push(codes[0].clone());
        recovery_codes.push(codes[1].clone());
        recovery_codes.push(codes[2].clone());
        recovery_codes.push(codes[3].clone());
    }

    transaction.commit().await?;

    Ok(ResetTotpResult { recovery_codes })
}
