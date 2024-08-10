use std::{path::PathBuf, sync::Arc};

use crate::{
    auth::AuthData,
    storage::{
        DirContentsError, EnsureExistsError, FileContentsError, StorageItem, StorageItemPath,
        UserStorage,
    },
    validate::PathValidationError,
    AppState,
};

pub struct UserDirectoryContentsResult {
    pub dir_contents: Vec<StorageItem>,
    pub path: StorageItemPath,
}

#[derive(thiserror::Error, Debug)]
pub enum UserDirectoryContentsError {
    #[error(transparent)]
    StorageEnsurance(#[from] EnsureExistsError),
    #[error(transparent)]
    DirContents(#[from] DirContentsError),
    #[error(transparent)]
    Validation(#[from] PathValidationError),
}

pub async fn user_directory_contents(
    app_state: &Arc<AppState>,
    auth_data: &AuthData,
    path: &str,
) -> Result<UserDirectoryContentsResult, UserDirectoryContentsError> {
    crate::validate::validate_file_path(path)?;

    let user_storage = UserStorage {
        user: auth_data.user.clone(),
        storage_root: app_state.config.fs_storage_root_dir.clone(),
    };

    user_storage.ensure_exists().await?;

    let path = StorageItemPath::new(user_storage.clone(), PathBuf::from(path));
    let mut dir_contents = user_storage.dir_contents(&path).await?;

    dir_contents.sort_by_key(|item| item.path.path());
    dir_contents.sort_by_key(|item| item.kind.clone());

    Ok(UserDirectoryContentsResult { dir_contents, path })
}

pub struct UserFileResult {
    pub file: tokio::fs::File,
    pub mime: mime_guess::Mime,
    pub path: StorageItemPath,
}

#[derive(thiserror::Error, Debug)]
pub enum UserFileError {
    #[error(transparent)]
    StorageEnsurance(#[from] EnsureExistsError),
    #[error(transparent)]
    FileContents(#[from] FileContentsError),
    #[error(transparent)]
    Validation(#[from] PathValidationError),
}

pub async fn user_file_download(
    app_state: &Arc<AppState>,
    auth_data: &AuthData,
    path: &str,
) -> Result<UserFileResult, UserFileError> {
    crate::validate::validate_file_path(path)?;

    let user_storage = UserStorage {
        user: auth_data.user.clone(),
        storage_root: app_state.config.fs_storage_root_dir.clone(),
    };

    user_storage.ensure_exists().await?;

    let path = StorageItemPath::new(user_storage.clone(), PathBuf::from(path));

    let mime = mime_guess::from_path(&path.scoped_path).first_or_octet_stream();
    let file = user_storage.file_contents(&path).await?;

    Ok(UserFileResult { file, mime, path })
}

// #[derive()]
// pub struct UserStorageItemSearchResultDirectory {
//     directory: DirItem,
//     score: u32,
// }

// #[derive()]
// pub struct UserStorageItemSearchResultFile {
//     file: FileItem,
//     score: u32,
// }

// #[derive()]
// pub struct UserStorageItemSearchResult {
//     directories: Vec<UserStorageItemSearchResultDirectory>,
//     files: Vec<UserStorageItemSearchResultFile>,
// }

// #[derive(thiserror::Error, Debug)]
// pub enum UserStorageItemSearchError {
//     #[error(transparent)]
//     StorageItemPathCreation(#[from] StorageItemPathError),
//     #[error(transparent)]
//     Storage(#[from] StorageError),
// }

// pub async fn user_storage_item_search<'context>(
//     app_state: &Arc<AppState>,
//     auth_data: &AuthData,
//     search: &str,
// ) -> Result<UserStorageItemSearchResult, UserStorageItemSearchError> {
//     let storage = Storage::create();

//     let user_storage = UserStorage {
//         user: auth_data.user.clone(),
//         storage_root: app_state.config.fs_storage_root_dir.clone(),
//     };
//     let path = StorageItemPath::new(user_storage, PathBuf::from("/"))?;

//     let storage_items = storage.list_storage_items_recursively(&path).await?;
//     let storage_paths: Vec<_> = storage_items
//         .iter()
//         .map(|item| item.path().to_string())
//         .collect();

//     let mut matcher = nucleo_matcher::Matcher::new(nucleo_matcher::Config::DEFAULT.match_paths());
//     let matches = nucleo_matcher::pattern::Pattern::parse(
//         search,
//         nucleo_matcher::pattern::CaseMatching::Smart,
//         nucleo_matcher::pattern::Normalization::Smart,
//     )
//     .match_list(&storage_paths, &mut matcher);

//     let mut storage_item_search_result = UserStorageItemSearchResult {
//         directories: Vec::new(),
//         files: Vec::new(),
//     };

//     for path_match in &matches {
//         for item in &storage_items {
//             if path_match.0 == &item.path().scoped_path.to_string_lossy().to_string() {
//                 match item {
//                     crate::storage::StorageItem::DirItem(dir_item) => storage_item_search_result
//                         .directories
//                         .push(UserStorageItemSearchResultDirectory {
//                             directory: dir_item.clone(),
//                             score: path_match.1,
//                         }),
//                     crate::storage::StorageItem::FileItem(file_item) => storage_item_search_result
//                         .files
//                         .push(UserStorageItemSearchResultFile {
//                             file: file_item.clone(),
//                             score: path_match.1,
//                         }),
//                 }
//             }
//         }
//     }

//     Ok(storage_item_search_result)
// }
