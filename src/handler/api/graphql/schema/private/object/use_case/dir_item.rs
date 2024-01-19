use std::{path::PathBuf, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    handler::api::graphql::PrivateContext,
    storage::{
        DirItem, DirItemContent, FileItem, Storage, StorageError, StorageItemPath,
        StorageItemPathError, StorageKind,
    },
};

#[derive(thiserror::Error, Debug)]
pub enum DirectoryReadError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error(transparent)]
    StorageItemPathCreation(#[from] StorageItemPathError),
    #[error(transparent)]
    Storage(#[from] StorageError),
}

pub async fn list_directories<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
    directory_content: Arc<Mutex<Option<DirItemContent>>>,
) -> Result<Vec<DirItem>, DirectoryReadError> {
    let mut content_lock = directory_content.lock().await;

    match content_lock.as_ref() {
        Some(content) => Ok(content.directories.clone()),
        None => {
            let context = ctx
                .data::<PrivateContext>()
                .map_err(DirectoryReadError::Context)?;

            let path = StorageItemPath::new(
                context.app_state.config.fs_storage_root_dir.clone(),
                PathBuf::from(path),
                context.current_user.id,
            )?;

            let storage = StorageKind::create(&context.app_state.config).await;
            let dir_content = storage.list_storage_items(&path).await?;

            let mut directories = Vec::new();
            let mut files = Vec::new();

            for item in dir_content {
                match item {
                    crate::storage::StorageItem::DirItem(dir_item) => directories.push(dir_item),
                    crate::storage::StorageItem::FileItem(file_item) => files.push(file_item),
                }
            }

            let dir_content = DirItemContent {
                files,
                directories: directories.clone(),
            };
            *content_lock = Some(dir_content);

            Ok(directories)
        }
    }
}

pub async fn list_files<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
    directory_content: Arc<Mutex<Option<DirItemContent>>>,
) -> Result<Vec<FileItem>, DirectoryReadError> {
    let mut content_lock = directory_content.lock().await;

    match content_lock.as_ref() {
        Some(content) => Ok(content.files.clone()),
        None => {
            let context = ctx
                .data::<PrivateContext>()
                .map_err(DirectoryReadError::Context)?;

            let path = StorageItemPath::new(
                context.app_state.config.fs_storage_root_dir.clone(),
                PathBuf::from(path),
                context.current_user.id,
            )?;

            let storage = StorageKind::create(&context.app_state.config).await;
            let dir_content = storage.list_storage_items(&path).await?;

            let mut directories = Vec::new();
            let mut files = Vec::new();

            for item in dir_content {
                match item {
                    crate::storage::StorageItem::DirItem(dir_item) => directories.push(dir_item),
                    crate::storage::StorageItem::FileItem(file_item) => files.push(file_item),
                }
            }

            let dir_content = DirItemContent {
                files: files.clone(),
                directories,
            };
            *content_lock = Some(dir_content);

            Ok(files)
        }
    }
}
