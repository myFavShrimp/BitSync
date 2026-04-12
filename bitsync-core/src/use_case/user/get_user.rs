use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::User,
    repository::{self, QueryError},
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("failed to get user")]
pub enum GetUserError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
}

pub async fn get_user(database: &Database, user_id: &Uuid) -> Result<User, GetUserError> {
    let mut connection = database.acquire_connection().await?;

    let user = repository::user::find_by_id(&mut *connection, user_id).await?;

    Ok(user)
}
