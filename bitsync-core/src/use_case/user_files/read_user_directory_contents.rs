use std::path::PathBuf;

use bitsync_database::entity::User;
use bitsync_storage::{
    operation::{
        read::{read_dir_contents, ReadDirContentsError},
        write::{ensure_user_storage_exists, EnsureUserStorageExistsError},
    },
    storage_item::StorageItem,
    storage_path::{StoragePath, StoragePathError},
    user_storage::UserStorage,
};

pub struct UserDirectoryContentsResult {
    pub dir_contents: Vec<StorageItem>,
    pub path: StoragePath,
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to read user directory contents")]
pub enum ReadUserDirectoryContentsError {
    EnsureUserStorageExists(#[from] EnsureUserStorageExistsError),
    ReadDirContents(#[from] ReadDirContentsError),
    StoragePath(#[from] StoragePathError),
}

pub async fn read_user_directory_contents(
    storage_root_dir: &PathBuf,
    path: &str,
    user: &User,
) -> Result<UserDirectoryContentsResult, ReadUserDirectoryContentsError> {
    let user_storage = UserStorage {
        user_id: user.id,
        storage_root: storage_root_dir.clone(),
    };

    ensure_user_storage_exists(&user_storage).await?;

    let path = StoragePath::new(user_storage.clone(), PathBuf::from(path))?;
    let mut dir_contents = read_dir_contents(&path).await?;

    dir_contents.sort_by_key(|item| item.path.path());
    dir_contents.sort_by_key(|item| item.kind.clone());

    Ok(UserDirectoryContentsResult { dir_contents, path })
}
