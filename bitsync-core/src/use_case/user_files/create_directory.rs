use std::path::PathBuf;

use bitsync_database::entity::User;
use bitsync_storage::{
    operation::{
        read::{read_dir_contents, ReadDirContentsError, ReadStorageItemError},
        write::{
            create_directory, ensure_user_storage_exists, CreateDirectoryError,
            EnsureUserStorageExistsError,
        },
    },
    storage_item::StorageItem,
    storage_path::{StoragePath, StoragePathError},
    user_storage::UserStorage,
    validation::{validate_path_is_just_file_name, PathIsJustFileNameValidationError},
};

pub struct DirectoryCreationResult {
    pub dir_contents: Vec<StorageItem>,
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to delete a user's file")]
pub enum UserFileDirecoryCreationError {
    EnsureUserStorageExists(#[from] EnsureUserStorageExistsError),
    StoragePath(#[from] StoragePathError),
    ReadStorageItem(#[from] ReadStorageItemError),
    DirectoryName(#[from] PathIsJustFileNameValidationError),
    CreateDirectory(#[from] CreateDirectoryError),
    ReadDirContents(#[from] ReadDirContentsError),
}

pub async fn create_direcory(
    storage_root_dir: &PathBuf,
    parent_directory: &str,
    direcory_name: &str,
    user: &User,
) -> Result<DirectoryCreationResult, UserFileDirecoryCreationError> {
    validate_path_is_just_file_name(direcory_name)?;

    let user_storage = UserStorage {
        user_id: user.id,
        storage_root: storage_root_dir.clone(),
    };

    ensure_user_storage_exists(&user_storage).await?;

    let mut directory_to_create = PathBuf::from(parent_directory);
    directory_to_create.push(direcory_name);

    let storage_path_to_create =
        StoragePath::new(user_storage.clone(), directory_to_create.clone())?;

    create_directory(&storage_path_to_create).await?;

    directory_to_create.pop();

    let directory_storage_path = StoragePath::new(user_storage.clone(), directory_to_create)?;
    let mut dir_contents = read_dir_contents(&directory_storage_path).await?;

    dir_contents.sort_by_key(|item| item.path.path());
    dir_contents.sort_by_key(|item| item.kind.clone());

    Ok(DirectoryCreationResult { dir_contents })
}
