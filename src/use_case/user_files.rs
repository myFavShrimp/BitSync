use std::{path::PathBuf, pin::Pin, sync::Arc};

use tokio::io::DuplexStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::{
    auth::AuthData,
    storage::{
        AsyncFileRead, DirContentsError, EnsureExistsError, FileContentsError, StorageBackend,
        StorageItem, StorageItemError, StorageItemPath, UserStorage,
    },
    validate::PathValidationError,
    AppState,
};

mod directory_zipping;

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

    StorageBackend::ensure_exists(&user_storage).await?;

    let path = StorageItemPath::new(user_storage.clone(), PathBuf::from(path));
    let mut dir_contents = StorageBackend::dir_contents(&path).await?;

    dir_contents.sort_by_key(|item| item.path.path());
    dir_contents.sort_by_key(|item| item.kind.clone());

    Ok(UserDirectoryContentsResult { dir_contents, path })
}

pub enum AsyncStorageItemRead {
    File(AsyncFileRead),
    Directory(DuplexStream),
}

impl tokio::io::AsyncRead for AsyncStorageItemRead {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let self_mut = self.get_mut();

        match self_mut {
            AsyncStorageItemRead::File(inner) => {
                let pinned_inner = Pin::new(inner);

                pinned_inner.poll_read(cx, buf)
            }
            AsyncStorageItemRead::Directory(inner) => {
                let pinned_inner = Pin::new(inner);

                pinned_inner.poll_read(cx, buf)
            }
        }
    }
}

pub struct UserFileResult {
    pub file: AsyncStorageItemRead,
    pub mime: mime_guess::Mime,
    pub path: StorageItemPath,
}

#[derive(thiserror::Error, Debug)]
#[error("An error occurred during user file download")]
pub enum UserFileError {
    StorageEnsurance(#[from] EnsureExistsError),
    FileContents(#[from] FileContentsError),
    Validation(#[from] PathValidationError),
    StorageItem(#[from] StorageItemError),
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

    StorageBackend::ensure_exists(&user_storage).await?;

    let path = StorageItemPath::new(user_storage.clone(), PathBuf::from(path));

    let storage_item = StorageBackend::storage_item(&path).await?;

    match storage_item.kind {
        crate::storage::StorageItemKind::File => {
            let mime = mime_guess::from_path(&path.scoped_path).first_or_octet_stream();
            let file = StorageBackend::file_stream(&path).await?;

            Ok(UserFileResult {
                file: AsyncStorageItemRead::File(file),
                mime,
                path,
            })
        }
        crate::storage::StorageItemKind::Directory => {
            let (write_stream, read_stream) = tokio::io::duplex(4096);

            tokio::spawn(async move {
                match directory_zipping::write_directory_zip_to_stream(write_stream, &storage_item)
                    .await
                {
                    Ok(_) => {}
                    Err(_) => todo!(),
                };
            });

            let mut dir_path = path.scoped_path.clone();
            dir_path.set_extension("zip");

            let fake_zip_path = StorageItemPath::new(user_storage.clone(), PathBuf::from(dir_path));

            let mime = mime_guess::from_path(&fake_zip_path.scoped_path).first_or_octet_stream();

            Ok(UserFileResult {
                file: AsyncStorageItemRead::Directory(read_stream),
                mime,
                path: fake_zip_path,
            })
        }
    }
}

// pub async fn user_file_delete(
//     app_state: &Arc<AppState>,
//     auth_data: &AuthData,
//     path: &str,
// ) -> Result<UserFileResult, UserFileError> {
//     crate::validate::validate_file_path(path)?;

//     let user_storage = UserStorage {
//         user: auth_data.user.clone(),
//         storage_root: app_state.config.fs_storage_root_dir.clone(),
//     };

//     StorageBackend::ensure_exists(&user_storage).await?;

//     let path = StorageItemPath::new(user_storage.clone(), PathBuf::from(path));

//     let mime = mime_guess::from_path(&path.scoped_path).first_or_octet_stream();
//     let file = StorageBackend::file_stream(&path).await?;

//     Ok(UserFileResult { file, mime, path })
// }

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
