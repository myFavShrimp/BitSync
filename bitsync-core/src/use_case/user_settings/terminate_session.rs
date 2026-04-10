use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::Session,
    repository,
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("Cannot terminate the current session")]
pub struct CannotTerminateCurrentSessionError;

#[derive(thiserror::Error, Debug)]
#[error("Failed to terminate session")]
pub enum TerminateSessionError {
    DatabaseQuery(#[from] repository::QueryError),
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    CannotTerminateCurrentSession(#[from] CannotTerminateCurrentSessionError),
}

pub async fn terminate_session(
    database: &Database,
    user_id: &Uuid,
    session_id_to_terminate: &Uuid,
    current_session_id: &Uuid,
) -> Result<Vec<Session>, TerminateSessionError> {
    if session_id_to_terminate == current_session_id {
        Err(CannotTerminateCurrentSessionError)?;
    }

    let mut connection = database.acquire_connection().await?;

    repository::session::delete_by_id(&mut *connection, session_id_to_terminate).await?;
    let sessions = repository::session::find_all_by_user_id(&mut *connection, user_id).await?;

    Ok(sessions)
}
