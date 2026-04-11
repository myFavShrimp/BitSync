use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::Session,
    repository,
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("failed to terminate session")]
pub enum TerminateSessionError {
    DatabaseQuery(#[from] repository::QueryError),
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
}

pub enum TerminateSessionOutcome {
    Terminated(Vec<Session>),
    CannotTerminateCurrentSession(Vec<Session>),
}

pub async fn terminate_session(
    database: &Database,
    user_id: &Uuid,
    session_id_to_terminate: &Uuid,
    current_session_id: &Uuid,
) -> Result<TerminateSessionOutcome, TerminateSessionError> {
    let mut connection = database.acquire_connection().await?;

    if session_id_to_terminate == current_session_id {
        let sessions = repository::session::find_all_by_user_id(&mut *connection, user_id).await?;

        return Ok(TerminateSessionOutcome::CannotTerminateCurrentSession(
            sessions,
        ));
    }

    repository::session::delete_by_id(&mut *connection, session_id_to_terminate).await?;
    let sessions = repository::session::find_all_by_user_id(&mut *connection, user_id).await?;

    Ok(TerminateSessionOutcome::Terminated(sessions))
}
