use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::UserShare,
    repository::{self, QueryError},
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("failed to create user share")]
pub enum CreateUserShareError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
}

pub async fn create_user_share(
    database: &Database,
    user_id: &Uuid,
    item_path: &str,
) -> Result<Vec<UserShare>, CreateUserShareError> {
    let mut connection = database.acquire_connection().await?;

    repository::user_share::create(&mut *connection, user_id, item_path).await?;

    let user_shares = repository::user_share::find_all_by_user_id_and_item_path(
        &mut *connection,
        user_id,
        item_path,
    )
    .await?;

    Ok(user_shares)
}
