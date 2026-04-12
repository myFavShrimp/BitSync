use std::path::Path;

use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::User,
    repository::{self, QueryError},
};
use bitsync_storage::{
    operation::write::{DeleteUserStorageError, delete_user_storage},
    user_storage::UserStorage,
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("failed to delete user")]
pub enum DeleteUserError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
    DeleteUserStorage(#[from] DeleteUserStorageError),
}

pub async fn delete_user(
    database: &Database,
    storage_root_dir: &Path,
    user_id: &Uuid,
) -> Result<Vec<User>, DeleteUserError> {
    let user_storage = UserStorage {
        user_id: *user_id,
        storage_root: storage_root_dir.to_path_buf(),
    };

    delete_user_storage(&user_storage).await?;

    let mut connection = database.acquire_connection().await?;
    repository::user::delete(&mut *connection, user_id).await?;

    let users = repository::user::find_all(&mut *connection).await?;

    Ok(users)
}
