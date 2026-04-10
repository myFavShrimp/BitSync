use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::Session,
    repository,
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("Failed to list sessions")]
pub enum ListSessionsError {
    DatabaseQuery(#[from] repository::QueryError),
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
}

pub async fn list_sessions(
    database: &Database,
    user_id: &Uuid,
) -> Result<Vec<Session>, ListSessionsError> {
    let mut connection = database.acquire_connection().await?;

    let sessions = repository::session::find_all_by_user_id(&mut *connection, user_id).await?;

    Ok(sessions)
}
