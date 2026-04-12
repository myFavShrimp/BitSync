use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::User,
    repository::{self, QueryError},
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("failed to list users")]
pub enum ListUsersError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
}

pub async fn list_users(
    database: &Database,
    current_user_id: &Uuid,
) -> Result<Vec<User>, ListUsersError> {
    let mut connection = database.acquire_connection().await?;

    let users = repository::user::find_all_except(&mut *connection, current_user_id).await?;

    Ok(users)
}
