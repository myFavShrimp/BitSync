use std::path::PathBuf;

use bitsync_database::entity::User;
use bitsync_storage::{
    operation::{
        read::{read_dir_contents, ReadDirContentsError, ReadStorageItemError},
        write::{
            ensure_user_storage_exists, rename_item, EnsureUserStorageExistsError, RenameItemError,
        },
    },
    storage_item::StorageItem,
    storage_path::{StoragePath, StoragePathError},
    user_storage::UserStorage,
};

pub struct UserFileMoveResult {
    pub dir_contents: Vec<StorageItem>,
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to delete a user's file")]
pub enum UserFileMoveError {
    EnsureUserStorageExists(#[from] EnsureUserStorageExistsError),
    StoragePath(#[from] StoragePathError),
    ReadStorageItem(#[from] ReadStorageItemError),
    RenameItem(#[from] RenameItemError),
    ReadDirContents(#[from] ReadDirContentsError),
}

pub async fn move_user_file(
    storage_root_dir: &PathBuf,
    path_to_move: &str,
    move_destination_path: &str,
    user: &User,
) -> Result<UserFileMoveResult, UserFileMoveError> {
    let user_storage = UserStorage {
        user_id: user.id,
        storage_root: storage_root_dir.clone(),
    };

    ensure_user_storage_exists(&user_storage).await?;

    let mut scoped_path_to_move = PathBuf::from(path_to_move);
    let scoped_move_destination_path = PathBuf::from(move_destination_path);

    let storage_path_to_move = StoragePath::new(user_storage.clone(), scoped_path_to_move.clone())?;
    let move_destination_storage_path =
        StoragePath::new(user_storage.clone(), scoped_move_destination_path)?;

    rename_item(&storage_path_to_move, &move_destination_storage_path).await?;

    scoped_path_to_move.pop();

    let directory_storage_path = StoragePath::new(user_storage.clone(), scoped_path_to_move)?;
    let mut dir_contents = read_dir_contents(&directory_storage_path).await?;

    dir_contents.sort_by_key(|item| item.path.path());
    dir_contents.sort_by_key(|item| item.kind.clone());

    Ok(UserFileMoveResult { dir_contents })
}
