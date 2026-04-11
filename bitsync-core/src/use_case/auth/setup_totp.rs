use bitsync_database::{
    database::{Database, TransactionBeginError, transaction::TransactionCommitError},
    entity::User,
    repository::{self, QueryError},
};

use crate::{
    hash::{PasswordHashCreationError, hash_password},
    jwt::{JwtClaims, LoginState},
    totp::{TotpCreationError, build_totp_for_user},
};

#[derive(thiserror::Error, Debug)]
#[error("failed to setup totp")]
pub enum TotpSetupError {
    TotpCreation(#[from] TotpCreationError),
    SystemTime(#[from] std::time::SystemTimeError),
    TotpInvalid(#[from] TotpInvalid),
    TransactionBegin(#[from] TransactionBeginError),
    TransactionCommit(#[from] TransactionCommitError),
    Query(#[from] QueryError),
    PasswordHashCreation(#[from] PasswordHashCreationError),
    TotpAlreadySetUp(#[from] TotpAlreadySetUp),
    Jwt(#[from] crate::jwt::Error),
}

#[derive(thiserror::Error, Debug)]
#[error("user provided an invalid totp value")]
pub struct TotpInvalid;

#[derive(thiserror::Error, Debug)]
#[error("totp is already set up")]
pub struct TotpAlreadySetUp;

pub struct TotpSetupResult {
    pub recovery_codes: Vec<String>,
    pub jwt: String,
}

const RECOVERY_CODE_COUNT: usize = 8;
const _: () = assert!(
    RECOVERY_CODE_COUNT.is_multiple_of(4),
    "count must be divisible by 4"
);

pub async fn setup_totp(
    database: &Database,
    user: &User,
    session_id: &uuid::Uuid,
    totp_value: &str,
    jwt_secret: &str,
) -> Result<TotpSetupResult, TotpSetupError> {
    if user.is_totp_set_up {
        return Err(TotpAlreadySetUp)?;
    }

    let totp = build_totp_for_user(user)?;

    if !totp.check_current(totp_value)? {
        return Err(TotpInvalid)?;
    }

    let mut transaction = database.begin_transaction().await?;
    repository::user::set_totp_setup_state(&mut *transaction, &user.id, true).await?;

    let mut recovery_codes = Vec::with_capacity(RECOVERY_CODE_COUNT);
    for _ in 0..(RECOVERY_CODE_COUNT / 4) {
        let codes = recovery_codes_for_user();

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

    let jwt = JwtClaims {
        sub: *session_id,
        login_state: LoginState::Full,
    }
    .encode(jwt_secret)?;

    Ok(TotpSetupResult {
        recovery_codes,
        jwt,
    })
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
