use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::Session,
    repository,
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("failed to terminate all other sessions")]
pub enum TerminateAllOtherSessionsError {
    DatabaseQuery(#[from] repository::QueryError),
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
}

pub async fn terminate_all_other_sessions(
    database: &Database,
    user_id: &Uuid,
    current_session_id: &Uuid,
) -> Result<Vec<Session>, TerminateAllOtherSessionsError> {
    let mut connection = database.acquire_connection().await?;

    repository::session::delete_all_by_user_id_except(
        &mut *connection,
        user_id,
        current_session_id,
    )
    .await?;

    let sessions = repository::session::find_all_by_user_id(&mut *connection, user_id).await?;

    Ok(sessions)
}
