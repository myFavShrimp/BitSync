use std::path::PathBuf;

use bitsync_database::entity::User;
use bitsync_storage::{
    operation::{
        read::{read_dir_contents, read_storage_item, ReadDirContentsError, ReadStorageItemError},
        write::{
            delete_directory, delete_file, ensure_user_storage_exists, DeleteDirectoryError,
            DeleteFileError, EnsureUserStorageExistsError,
        },
    },
    storage_item::{StorageItem, StorageItemKind},
    storage_path::{StoragePath, StoragePathError},
    user_storage::UserStorage,
};

pub struct UserFileDeletionResult {
    pub dir_contents: Vec<StorageItem>,
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to delete a user's file")]
pub enum UserFileDeletionError {
    EnsureUserStorageExists(#[from] EnsureUserStorageExistsError),
    StoragePath(#[from] StoragePathError),
    ReadStorageItem(#[from] ReadStorageItemError),
    DeleteDirectory(#[from] DeleteDirectoryError),
    DeleteFile(#[from] DeleteFileError),
    ReadDirContents(#[from] ReadDirContentsError),
}

pub async fn delete_user_file(
    storage_root_dir: &PathBuf,
    path: &str,
    user: &User,
) -> Result<UserFileDeletionResult, UserFileDeletionError> {
    let user_storage = UserStorage {
        user_id: user.id,
        storage_root: storage_root_dir.clone(),
    };

    ensure_user_storage_exists(&user_storage).await?;

    let mut scoped_path = PathBuf::from(path);

    let storage_path_to_delete = StoragePath::new(user_storage.clone(), scoped_path.clone())?;
    let storage_item = read_storage_item(&storage_path_to_delete).await?;

    match storage_item.kind {
        StorageItemKind::Directory => {
            delete_directory(&storage_path_to_delete).await?;
        }
        StorageItemKind::File => {
            delete_file(&storage_path_to_delete).await?;
        }
    }

    scoped_path.pop();

    let directory_storage_path = StoragePath::new(user_storage.clone(), scoped_path)?;
    let mut dir_contents = read_dir_contents(&directory_storage_path).await?;

    dir_contents.sort_by_key(|item| item.path.path());
    dir_contents.sort_by_key(|item| item.kind.clone());

    Ok(UserFileDeletionResult { dir_contents })
}
