use std::path::PathBuf;

use async_graphql::Upload;

use crate::{
    handler::api::graphql::PrivateContext,
    storage::{
        DirItem, FileItem, Storage, StorageError, StorageItem, StorageItemPath,
        StorageItemPathError, StorageKind, UserStorage,
    },
};

#[derive(thiserror::Error, Debug)]
pub enum UserFileUploadError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error(transparent)]
    StorageItemPathCreation(#[from] StorageItemPathError),
    #[error("Error handling the file upload")]
    Storage(#[from] StorageError),
    #[error("Could not read the file data")]
    FileUploadRead(std::io::Error),
}

pub async fn upload_user_files<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
    mut files: Vec<Upload>,
) -> Result<Vec<FileItem>, UserFileUploadError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserFileUploadError::Context)?;

    let files: Result<Vec<async_graphql::UploadValue>, std::io::Error> =
        files.iter_mut().map(|file| file.value(ctx)).collect();

    let user_storage = UserStorage {
        user: context.current_user.clone(),
        storage_root: context.app_state.config.fs_storage_root_dir.clone(),
    };
    let path = StorageItemPath::new(user_storage, PathBuf::from(path))?;

    let storage = StorageKind::create(&context.app_state.config).await;

    let mut result = Vec::new();
    for file in files.map_err(UserFileUploadError::FileUploadRead)? {
        let file_content = file.content;

        let mut file_path = path.clone();
        file_path.push(&file.filename);

        result.push(storage.add_file(&file_path, file_content).await?);
    }

    Ok(result)
}

#[derive(thiserror::Error, Debug)]
pub enum UserStorageItemMoveError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error(transparent)]
    StorageItemPathCreation(#[from] StorageItemPathError),
    #[error("Error handling the rename operation")]
    Storage(#[from] StorageError),
}

pub async fn move_user_storage_item<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
    new_path: &str,
) -> Result<StorageItem, UserStorageItemMoveError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserStorageItemMoveError::Context)?;

    let user_storage = UserStorage {
        user: context.current_user.clone(),
        storage_root: context.app_state.config.fs_storage_root_dir.clone(),
    };
    let path = StorageItemPath::new(user_storage.clone(), PathBuf::from(path))?;
    let new_path = StorageItemPath::new(user_storage, PathBuf::from(new_path))?;

    let storage = StorageKind::create(&context.app_state.config).await;

    Ok(storage.move_item(&path, &new_path).await?)
}

#[derive(thiserror::Error, Debug)]
pub enum UserFileCopyError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error(transparent)]
    StorageItemPathCreation(#[from] StorageItemPathError),
    #[error("Error handling the copy operation")]
    Storage(#[from] StorageError),
}

pub async fn copy_user_file<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
    new_path: &str,
) -> Result<FileItem, UserFileCopyError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserFileCopyError::Context)?;

    let user_storage = UserStorage {
        user: context.current_user.clone(),
        storage_root: context.app_state.config.fs_storage_root_dir.clone(),
    };
    let path = StorageItemPath::new(user_storage.clone(), PathBuf::from(path))?;
    let new_path = StorageItemPath::new(user_storage, PathBuf::from(new_path))?;

    let storage = StorageKind::create(&context.app_state.config).await;

    Ok(storage.copy_file(&path, &new_path).await?)
}

#[derive(thiserror::Error, Debug)]
pub enum UserDirectoryCopyError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error(transparent)]
    StorageItemPathCreation(#[from] StorageItemPathError),
    #[error("Error handling the copy operation")]
    Storage(#[from] StorageError),
}

pub async fn copy_user_directory<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
    new_path: &str,
) -> Result<DirItem, UserDirectoryCopyError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserDirectoryCopyError::Context)?;

    let user_storage = UserStorage {
        user: context.current_user.clone(),
        storage_root: context.app_state.config.fs_storage_root_dir.clone(),
    };
    let path = StorageItemPath::new(user_storage.clone(), PathBuf::from(path))?;
    let new_path = StorageItemPath::new(user_storage, PathBuf::from(new_path))?;

    let storage = StorageKind::create(&context.app_state.config).await;

    Ok(storage.copy_directory(&path, &new_path).await?)
}

#[derive(thiserror::Error, Debug)]
pub enum UserDirectoryCreationError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error(transparent)]
    StorageItemPathCreation(#[from] StorageItemPathError),
    #[error("Error handling the create operation")]
    Storage(#[from] StorageError),
    #[error("Already exists")]
    AlreadyExists,
}

pub async fn create_user_directory<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
) -> Result<DirItem, UserDirectoryCreationError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserDirectoryCreationError::Context)?;

    let user_storage = UserStorage {
        user: context.current_user.clone(),
        storage_root: context.app_state.config.fs_storage_root_dir.clone(),
    };
    let path = StorageItemPath::new(user_storage.clone(), PathBuf::from(path))?;

    let storage = StorageKind::create(&context.app_state.config).await;

    if let Ok(crate::storage::StorageItem::DirItem(_)) = storage.storage_item(&path).await {
        Err(UserDirectoryCreationError::AlreadyExists)?
    }

    Ok(storage.create_directory(&path).await?)
}

#[derive(thiserror::Error, Debug)]
pub enum UserDirectoryRemovalError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error(transparent)]
    StorageItemPathCreation(#[from] StorageItemPathError),
    #[error("Error handling the remove operation")]
    Storage(#[from] StorageError),
}

pub async fn remove_user_directory<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
) -> Result<StorageItemPath, UserDirectoryRemovalError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserDirectoryRemovalError::Context)?;

    let user_storage = UserStorage {
        user: context.current_user.clone(),
        storage_root: context.app_state.config.fs_storage_root_dir.clone(),
    };
    let path = StorageItemPath::new(user_storage.clone(), PathBuf::from(path))?;

    if path.data_directory() == user_storage.data_directory() {
        todo!("deleting the user's directory should not be allowed")
    }

    let storage = StorageKind::create(&context.app_state.config).await;

    storage.remove_directory(&path).await?;

    Ok(path)
}

#[derive(thiserror::Error, Debug)]
pub enum UserFileRemovalError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error(transparent)]
    StorageItemPathCreation(#[from] StorageItemPathError),
    #[error("Error handling the remove operation")]
    Storage(#[from] StorageError),
}

pub async fn remove_user_file<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
) -> Result<StorageItemPath, UserFileRemovalError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserFileRemovalError::Context)?;

    let user_storage = UserStorage {
        user: context.current_user.clone(),
        storage_root: context.app_state.config.fs_storage_root_dir.clone(),
    };
    let path = StorageItemPath::new(user_storage.clone(), PathBuf::from(path))?;

    let storage = StorageKind::create(&context.app_state.config).await;

    storage.remove_file(&path).await?;

    Ok(path)
}
