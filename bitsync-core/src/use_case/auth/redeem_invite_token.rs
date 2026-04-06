use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    repository,
};

use super::InvalidInviteTokenError;

#[derive(thiserror::Error, Debug)]
#[error("failed to redeem invite token")]
pub enum RedeemInviteTokenError {
    DatabaseQuery(#[from] repository::QueryError),
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    InvalidInviteTokenError(#[from] InvalidInviteTokenError),
}

pub async fn redeem_invite_token(
    database: &Database,
    invite_token_value: &str,
) -> Result<(), RedeemInviteTokenError> {
    let invite_token_id =
        uuid::Uuid::parse_str(invite_token_value).map_err(|_| InvalidInviteTokenError)?;

    let mut connection = database.acquire_connection().await?;

    let token = repository::invite_token::find_by_id(&mut *connection, &invite_token_id).await?;

    if token.is_none() {
        Err(InvalidInviteTokenError)?;
    }

    Ok(())
}
