use std::path::PathBuf;

use async_graphql::Upload;

use crate::{
    handler::api::graphql::PrivateContext,
    storage::{
        DirItem, FileItem, Storage, StorageError, StorageItem, StorageItemPath,
        StorageItemPathError,
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

    let path = StorageItemPath::new(
        context.app_state.config.fs_storage_root_dir.clone(),
        PathBuf::from(path),
        context.current_user.id,
    )?;

    let storage = Storage;

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
pub enum UserDirectoryMoveError {
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
) -> Result<StorageItem, UserDirectoryMoveError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserDirectoryMoveError::Context)?;

    let path = StorageItemPath::new(
        context.app_state.config.fs_storage_root_dir.clone(),
        PathBuf::from(path),
        context.current_user.id,
    )?;

    let new_path = StorageItemPath::new(
        context.app_state.config.fs_storage_root_dir.clone(),
        PathBuf::from(new_path),
        context.current_user.id,
    )?;

    let storage = Storage;

    Ok(storage.move_item(&path, &new_path).await?)
}

#[derive(thiserror::Error, Debug)]
pub enum UserDirectoryCreationError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error(transparent)]
    StorageItemPathCreation(#[from] StorageItemPathError),
    #[error("Error handling the rename operation")]
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

    let path = StorageItemPath::new(
        context.app_state.config.fs_storage_root_dir.clone(),
        PathBuf::from(path),
        context.current_user.id,
    )?;

    let storage = Storage;

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

    let path = StorageItemPath::new(
        context.app_state.config.fs_storage_root_dir.clone(),
        PathBuf::from(path),
        context.current_user.id,
    )?;

    let storage = Storage;

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

    let path = StorageItemPath::new(
        context.app_state.config.fs_storage_root_dir.clone(),
        PathBuf::from(path),
        context.current_user.id,
    )?;

    let storage = Storage;

    storage.remove_file(&path).await?;

    Ok(path)
}
