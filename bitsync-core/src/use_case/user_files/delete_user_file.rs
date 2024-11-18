use std::path::PathBuf;

use bitsync_database::entity::User;
use bitsync_storage::{
    operation::{
        read::{read_storage_item, ReadStorageItemError},
        write::{
            delete_directory, delete_file, ensure_user_storage_exists, DeleteDirectoryError,
            DeleteFileError, EnsureUserStorageExistsError,
        },
    },
    storage_item::StorageItemKind,
    storage_path::{StoragePath, StoragePathError},
    user_storage::UserStorage,
};

#[derive(thiserror::Error, Debug)]
#[error("Failed to delete a user's file")]
pub enum UserFileDeletionError {
    EnsureUserStorageExists(#[from] EnsureUserStorageExistsError),
    StoragePath(#[from] StoragePathError),
    ReadStorageItem(#[from] ReadStorageItemError),
    DeleteDirectory(#[from] DeleteDirectoryError),
    DeleteFile(#[from] DeleteFileError),
}

pub async fn delete_user_file(
    storage_root_dir: &PathBuf,
    path: &str,
    user: &User,
) -> Result<(), UserFileDeletionError> {
    let user_storage = UserStorage {
        user_id: user.id,
        storage_root: storage_root_dir.clone(),
    };

    ensure_user_storage_exists(&user_storage).await?;

    let path = StoragePath::new(user_storage.clone(), PathBuf::from(path))?;
    let storage_item = read_storage_item(&path).await?;

    match storage_item.kind {
        StorageItemKind::Directory => {
            delete_directory(&path).await?;
        }
        StorageItemKind::File => {
            delete_file(&path).await?;
        }
    }

    Ok(())
}
