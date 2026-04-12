use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::User,
    repository::{self, QueryError},
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("failed to make user admin")]
pub enum MakeAdminError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
}

pub async fn make_admin(
    database: &Database,
    user_id: &Uuid,
    current_user_id: &Uuid,
) -> Result<Vec<User>, MakeAdminError> {
    let mut connection = database.acquire_connection().await?;

    repository::user::set_admin(&mut *connection, user_id, true).await?;

    let users = repository::user::find_all_except(&mut *connection, current_user_id).await?;

    Ok(users)
}
