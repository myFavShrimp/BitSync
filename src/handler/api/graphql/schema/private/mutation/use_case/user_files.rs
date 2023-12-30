use async_graphql::Upload;

use crate::{
    handler::api::graphql::PrivateContext,
    storage::{user_data_directory, DirItem, FileItem, Storage, StorageError},
};

#[derive(thiserror::Error, Debug)]
pub enum UserFileUploadError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
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

    let user_directory = user_data_directory(
        context.app_state.config.fs_storage_root_dir.clone(),
        &context.current_user.id,
    );

    let files: Result<Vec<async_graphql::UploadValue>, std::io::Error> =
        files.iter_mut().map(|file| file.value(ctx)).collect();

    let storage = Storage {
        storage_root: user_directory,
    };

    let mut result = Vec::new();
    for file in files.map_err(UserFileUploadError::FileUploadRead)? {
        let file_name = &file.filename;
        let file_content = file.content;

        result.push(storage.add_file(path, file_name, file_content).await?);
    }

    Ok(result)
}

#[derive(thiserror::Error, Debug)]
pub enum UserDirectoryMoveError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error("Error handling the rename operation")]
    Storage(#[from] StorageError),
    #[error("Not a directory")]
    NotADirectory,
}

pub async fn move_user_directory_item<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
    new_path: &str,
) -> Result<FileItem, UserDirectoryMoveError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserDirectoryMoveError::Context)?;

    let user_directory = user_data_directory(
        context.app_state.config.fs_storage_root_dir.clone(),
        &context.current_user.id,
    );

    let storage = Storage {
        storage_root: user_directory,
    };

    match storage.storage_item(path).await? {
        crate::storage::StorageItem::DirItem(_) => {}
        crate::storage::StorageItem::FileItem(_) => Err(UserDirectoryMoveError::NotADirectory)?,
    }

    Ok(storage.move_item(path, new_path).await?)
}

#[derive(thiserror::Error, Debug)]
pub enum UserFileMoveError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error("Error handling the rename operation")]
    Storage(#[from] StorageError),
    #[error("Not a file")]
    NotAFile,
}

pub async fn move_user_file_item<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
    new_path: &str,
) -> Result<FileItem, UserFileMoveError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserFileMoveError::Context)?;

    let user_directory = user_data_directory(
        context.app_state.config.fs_storage_root_dir.clone(),
        &context.current_user.id,
    );

    let storage = Storage {
        storage_root: user_directory,
    };

    match storage.storage_item(path).await? {
        crate::storage::StorageItem::DirItem(_) => Err(UserFileMoveError::NotAFile)?,
        crate::storage::StorageItem::FileItem(_) => {}
    }

    Ok(storage.move_item(path, new_path).await?)
}

#[derive(thiserror::Error, Debug)]
pub enum UserDirectoryCreationError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
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

    let user_directory = user_data_directory(
        context.app_state.config.fs_storage_root_dir.clone(),
        &context.current_user.id,
    );

    let storage = Storage {
        storage_root: user_directory,
    };

    match storage.storage_item(path).await? {
        crate::storage::StorageItem::DirItem(_) => Err(UserDirectoryCreationError::AlreadyExists)?,
        crate::storage::StorageItem::FileItem(_) => {}
    }

    Ok(storage.create_directory(path).await?)
}
