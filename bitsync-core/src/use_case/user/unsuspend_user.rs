use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::User,
    repository::{self, QueryError},
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("failed to unsuspend user")]
pub enum UnsuspendUserError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
}

pub async fn unsuspend_user(
    database: &Database,
    user_id: &Uuid,
    current_user_id: &Uuid,
) -> Result<Vec<User>, UnsuspendUserError> {
    let mut connection = database.acquire_connection().await?;

    repository::user::set_suspended(&mut *connection, user_id, false).await?;

    let users = repository::user::find_all_except(&mut *connection, current_user_id).await?;

    Ok(users)
}
