use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::User,
    repository::{self, QueryError},
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("failed to revoke admin from user")]
pub enum RevokeAdminError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
}

pub async fn revoke_admin(
    database: &Database,
    user_id: &Uuid,
) -> Result<Vec<User>, RevokeAdminError> {
    let mut connection = database.acquire_connection().await?;

    repository::user::set_admin(&mut *connection, user_id, false).await?;

    let users = repository::user::find_all(&mut *connection).await?;

    Ok(users)
}
