use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::InviteToken,
    repository::{self, QueryError},
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("failed to delete invite token")]
pub enum DeleteInviteTokenError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
}

pub async fn delete_invite_token(
    database: &Database,
    invite_token_id: &Uuid,
) -> Result<Vec<InviteToken>, DeleteInviteTokenError> {
    let mut connection = database.acquire_connection().await?;

    repository::invite_token::delete_by_id(&mut *connection, invite_token_id).await?;
    let invite_tokens = repository::invite_token::find_all(&mut *connection).await?;

    Ok(invite_tokens)
}
