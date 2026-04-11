use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    repository,
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("failed to logout")]
pub enum LogoutError {
    DatabaseQuery(#[from] repository::QueryError),
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
}

pub async fn logout(database: &Database, session_id: &Uuid) -> Result<(), LogoutError> {
    let mut connection = database.acquire_connection().await?;

    repository::session::delete_by_id(&mut *connection, session_id).await?;

    Ok(())
}
