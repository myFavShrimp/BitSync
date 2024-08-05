use std::{path::PathBuf, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    auth::AuthData,
    storage::{
        DirItem, DirItemContent, FileItem, Storage, StorageError, StorageItemPath,
        StorageItemPathError, UserStorage,
    },
    AppState,
};

#[derive(thiserror::Error, Debug)]
pub enum DirectoryReadError {
    #[error(transparent)]
    StorageItemPathCreation(#[from] StorageItemPathError),
    #[error(transparent)]
    Storage(#[from] StorageError),
}

pub async fn list_directories<'context>(
    app_state: &Arc<AppState>,
    auth_data: &AuthData,
    path: &str,
    directory_content: Arc<Mutex<Option<DirItemContent>>>,
) -> Result<Vec<DirItem>, DirectoryReadError> {
    let mut content_lock = directory_content.lock().await;

    match content_lock.as_ref() {
        Some(content) => Ok(content.directories.clone()),
        None => {
            let user_storage = UserStorage {
                user: auth_data.user.clone(),
                storage_root: app_state.config.fs_storage_root_dir.clone(),
            };
            let path = StorageItemPath::new(user_storage, PathBuf::from(path))?;

            let storage = Storage::create();
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
    app_state: &Arc<AppState>,
    auth_data: &AuthData,
    path: &str,
    directory_content: Arc<Mutex<Option<DirItemContent>>>,
) -> Result<Vec<FileItem>, DirectoryReadError> {
    let mut content_lock = directory_content.lock().await;

    match content_lock.as_ref() {
        Some(content) => Ok(content.files.clone()),
        None => {
            let user_storage = UserStorage {
                user: auth_data.user.clone(),
                storage_root: app_state.config.fs_storage_root_dir.clone(),
            };
            let path = StorageItemPath::new(user_storage, PathBuf::from(path))?;

            let storage = Storage::create();
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
