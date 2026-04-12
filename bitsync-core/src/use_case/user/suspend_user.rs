use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::User,
    repository::{self, QueryError},
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("failed to suspend user")]
pub enum SuspendUserError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
}

pub async fn suspend_user(
    database: &Database,
    user_id: &Uuid,
) -> Result<Vec<User>, SuspendUserError> {
    let mut connection = database.acquire_connection().await?;

    repository::user::set_suspended(&mut *connection, user_id, true).await?;

    let users = repository::user::find_all(&mut *connection).await?;

    Ok(users)
}
