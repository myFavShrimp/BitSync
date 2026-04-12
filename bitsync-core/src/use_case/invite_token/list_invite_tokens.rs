use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::InviteToken,
    repository::{self, QueryError},
};

#[derive(thiserror::Error, Debug)]
#[error("failed to list invite tokens")]
pub enum ListInviteTokensError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
}

pub async fn list_invite_tokens(
    database: &Database,
) -> Result<Vec<InviteToken>, ListInviteTokensError> {
    let mut connection = database.acquire_connection().await?;

    let invite_tokens = repository::invite_token::find_all(&mut *connection).await?;

    Ok(invite_tokens)
}
