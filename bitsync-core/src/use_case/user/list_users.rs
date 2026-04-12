use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::User,
    repository::{self, QueryError},
};

#[derive(thiserror::Error, Debug)]
#[error("failed to list users")]
pub enum ListUsersError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
}

pub async fn list_users(database: &Database) -> Result<Vec<User>, ListUsersError> {
    let mut connection = database.acquire_connection().await?;

    let users = repository::user::find_all(&mut *connection).await?;

    Ok(users)
}
