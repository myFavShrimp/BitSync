use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::InviteToken,
    repository::{self, QueryError},
};

#[derive(thiserror::Error, Debug)]
#[error("failed to create invite token")]
pub enum CreateInviteTokenError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
}

pub async fn create_invite_token(
    database: &Database,
) -> Result<Vec<InviteToken>, CreateInviteTokenError> {
    let mut connection = database.acquire_connection().await?;

    repository::invite_token::create(&mut *connection).await?;
    let invite_tokens = repository::invite_token::find_all(&mut *connection).await?;

    Ok(invite_tokens)
}
