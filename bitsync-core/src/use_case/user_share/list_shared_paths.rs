use std::path::{Path, PathBuf};

use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::User,
    repository::{self, QueryError},
};
use bitsync_storage::{
    operation::read::read_storage_item, storage_item::StorageItemKind, storage_path::StoragePath,
    user_storage::UserStorage,
};

#[derive(Clone)]
pub struct SharedPath {
    pub path: String,
    pub kind: StorageItemKind,
}

#[derive(thiserror::Error, Debug)]
#[error("failed to list shared paths")]
pub enum ListSharedPathsError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Query(#[from] QueryError),
}

pub async fn list_shared_paths(
    database: &Database,
    storage_root_dir: &Path,
    user: &User,
) -> Result<Vec<SharedPath>, ListSharedPathsError> {
    let mut connection = database.acquire_connection().await?;

    let item_paths =
        repository::user_share::find_distinct_item_paths_by_user_id(&mut *connection, &user.id)
            .await?;

    let user_storage = UserStorage {
        user_id: user.id,
        storage_root: storage_root_dir.to_path_buf(),
    };

    let mut shared_paths = Vec::with_capacity(item_paths.len());

    for item_path in item_paths {
        let kind = match StoragePath::new(user_storage.clone(), PathBuf::from(&item_path)) {
            Ok(storage_path) => match read_storage_item(&storage_path).await {
                Ok(storage_item) => storage_item.kind,
                Err(_) => StorageItemKind::File,
            },
            Err(_) => StorageItemKind::File,
        };

        shared_paths.push(SharedPath {
            path: item_path,
            kind,
        });
    }

    Ok(shared_paths)
}
