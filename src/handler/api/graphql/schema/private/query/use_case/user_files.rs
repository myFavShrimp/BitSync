use std::path::PathBuf;

use crate::{
    handler::api::graphql::PrivateContext,
    storage::{
        DirItem, FileItem, Storage, StorageError, StorageItemPath, StorageItemPathError,
        StorageKind,
    },
};

#[derive(thiserror::Error, Debug)]
pub enum UserDirectoryReadError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error(transparent)]
    StorageItemPathCreation(#[from] StorageItemPathError),
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error("Not a directory")]
    NotADirectory,
}

pub async fn user_directory<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
) -> Result<DirItem, UserDirectoryReadError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserDirectoryReadError::Context)?;

    let storage = StorageKind::create(&context.app_state.config).await;

    let path = StorageItemPath::new(
        context.app_state.config.fs_storage_root_dir.clone(),
        PathBuf::from(path),
        context.current_user.id,
    )?;

    let storage_item = storage.storage_item(&path).await?;

    match storage_item {
        crate::storage::StorageItem::FileItem(_) => Err(UserDirectoryReadError::NotADirectory),
        crate::storage::StorageItem::DirItem(dir_item) => Ok(dir_item),
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct UserStorageItemSearchResultDirectory {
    directory: DirItem,
    score: u32,
}

#[derive(async_graphql::SimpleObject)]
pub struct UserStorageItemSearchResultFile {
    file: FileItem,
    score: u32,
}

#[derive(async_graphql::SimpleObject)]
pub struct UserStorageItemSearchResult {
    directories: Vec<UserStorageItemSearchResultDirectory>,
    files: Vec<UserStorageItemSearchResultFile>,
}

#[derive(thiserror::Error, Debug)]
pub enum UserStorageItemSearchError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error(transparent)]
    StorageItemPathCreation(#[from] StorageItemPathError),
    #[error(transparent)]
    Storage(#[from] StorageError),
}

pub async fn user_storage_item_search<'context>(
    ctx: &async_graphql::Context<'context>,
    search: &str,
) -> Result<UserStorageItemSearchResult, UserStorageItemSearchError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserStorageItemSearchError::Context)?;

    let storage = StorageKind::create(&context.app_state.config).await;

    let path = StorageItemPath::new(
        context.app_state.config.fs_storage_root_dir.clone(),
        PathBuf::from("/"),
        context.current_user.id,
    )?;

    let storage_items = storage.list_storage_items_recursively(&path).await?;
    let storage_paths: Vec<_> = storage_items
        .iter()
        .map(|item| item.path().to_string())
        .collect();

    let mut matcher = nucleo_matcher::Matcher::new(nucleo_matcher::Config::DEFAULT.match_paths());
    let matches = nucleo_matcher::pattern::Pattern::parse(
        search,
        nucleo_matcher::pattern::CaseMatching::Smart,
        nucleo_matcher::pattern::Normalization::Smart,
    )
    .match_list(&storage_paths, &mut matcher);

    let mut storage_item_search_result = UserStorageItemSearchResult {
        directories: Vec::new(),
        files: Vec::new(),
    };

    for path_match in &matches {
        for item in &storage_items {
            if path_match.0 == &item.path().scoped_path.to_string_lossy().to_string() {
                match item {
                    crate::storage::StorageItem::DirItem(dir_item) => storage_item_search_result
                        .directories
                        .push(UserStorageItemSearchResultDirectory {
                            directory: dir_item.clone(),
                            score: path_match.1,
                        }),
                    crate::storage::StorageItem::FileItem(file_item) => storage_item_search_result
                        .files
                        .push(UserStorageItemSearchResultFile {
                            file: file_item.clone(),
                            score: path_match.1,
                        }),
                }
            }
        }
    }

    Ok(storage_item_search_result)
}
