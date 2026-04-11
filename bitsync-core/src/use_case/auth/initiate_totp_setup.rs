use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    repository::{self, QueryError},
};
use uuid::Uuid;

use crate::{
    random::GenerateRandomBytesError,
    totp::{BuildTotpSetupDataError, TotpSetupData, build_totp_setup_data, generate_totp_secret},
};

#[derive(thiserror::Error, Debug)]
#[error("failed to initiate totp setup")]
pub enum InitiateTotpSetupError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
    GenerateRandomBytes(#[from] GenerateRandomBytesError),
    BuildTotpSetupData(#[from] BuildTotpSetupDataError),
}

pub async fn initiate_totp_setup(
    database: &Database,
    user_id: &Uuid,
) -> Result<TotpSetupData, InitiateTotpSetupError> {
    let mut connection = database.acquire_connection().await?;

    let dangling_totp_secret = generate_totp_secret()?;
    let user = repository::user::set_dangling_totp_secret(
        &mut *connection,
        user_id,
        &dangling_totp_secret,
    )
    .await?;

    let setup_data = build_totp_setup_data(&dangling_totp_secret, &user.username)?;

    Ok(setup_data)
}
