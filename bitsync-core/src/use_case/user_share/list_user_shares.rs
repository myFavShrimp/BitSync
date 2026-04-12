use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::UserShare,
    repository::{self, QueryError},
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("failed to list user shares")]
pub enum ListUserSharesError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
}

pub async fn list_user_shares(
    database: &Database,
    user_id: &Uuid,
    item_path: &str,
) -> Result<Vec<UserShare>, ListUserSharesError> {
    let mut connection = database.acquire_connection().await?;

    let user_shares = repository::user_share::find_all_by_user_id_and_item_path(
        &mut *connection,
        user_id,
        item_path,
    )
    .await?;

    Ok(user_shares)
}
