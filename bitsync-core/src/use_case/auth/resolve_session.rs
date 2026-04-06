use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::{Session, User},
    repository,
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("Failed to resolve session")]
pub enum ResolveSessionError {
    DatabaseQuery(#[from] repository::QueryError),
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
}

pub struct ResolveSessionResult {
    pub session: Session,
    pub user: User,
}

pub async fn resolve_session(
    database: &Database,
    session_id: &Uuid,
) -> Result<ResolveSessionResult, ResolveSessionError> {
    let mut connection = database.acquire_connection().await?;

    let session = repository::session::touch(&mut *connection, session_id).await?;
    let user = repository::user::find_by_id(&mut *connection, &session.user_id).await?;

    Ok(ResolveSessionResult { session, user })
}
