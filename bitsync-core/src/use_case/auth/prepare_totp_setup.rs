use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::User,
    repository::{self, QueryError},
};
use uuid::Uuid;

use crate::{
    random::GenerateRandomBytesError,
    totp::{BuildTotpSetupDataError, TotpSetupData, build_totp_setup_data, generate_totp_secret},
};

#[derive(thiserror::Error, Debug)]
#[error("failed to prepare totp setup")]
pub enum PrepareTotpSetupError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
    GenerateRandomBytes(#[from] GenerateRandomBytesError),
    BuildTotpSetupData(#[from] BuildTotpSetupDataError),
}

pub async fn prepare_totp_setup(
    database: &Database,
    user: &User,
) -> Result<TotpSetupData, PrepareTotpSetupError> {
    let dangling_secret = match &user.dangling_totp_secret {
        Some(secret) => secret.clone(),
        None => ensure_dangling_totp_secret(database, &user.id).await?,
    };

    let setup_data = build_totp_setup_data(&dangling_secret, &user.username)?;

    Ok(setup_data)
}

async fn ensure_dangling_totp_secret(
    database: &Database,
    user_id: &Uuid,
) -> Result<Vec<u8>, PrepareTotpSetupError> {
    let mut connection = database.acquire_connection().await?;

    let dangling_totp_secret = generate_totp_secret()?;
    repository::user::set_dangling_totp_secret(&mut *connection, user_id, &dangling_totp_secret)
        .await?;

    Ok(dangling_totp_secret)
}
