use std::path::{Path, PathBuf};

use bitsync_database::entity::User;
use bitsync_storage::{
    operation::{
        read::{ReadDirContentsError, ReadStorageItemError, read_dir_contents, read_storage_item},
        write::{EnsureUserStorageExistsError, ensure_user_storage_exists},
    },
    storage_item::{StorageItem, StorageItemKind},
    storage_path::{StoragePath, StoragePathError},
    user_storage::UserStorage,
};

use super::shared::user_root_directory_name;

pub struct DirectoryBreadcrumbSegment {
    pub name: String,
    pub path: String,
}

pub struct UserDirectoryContentsResult {
    pub dir_contents: Vec<StorageItem>,
    pub path: StoragePath,
    pub directory_name: String,
    pub is_root_directory: bool,
    pub breadcrumb_segments: Vec<DirectoryBreadcrumbSegment>,
}

pub struct UserFileItemResult {
    pub path: StoragePath,
    pub size: u64,
    pub file_name: String,
    pub breadcrumb_segments: Vec<DirectoryBreadcrumbSegment>,
}

pub enum UserFilesHomeResult {
    Directory(UserDirectoryContentsResult),
    File(UserFileItemResult),
}

#[derive(thiserror::Error, Debug)]
#[error("failed to read user file item")]
pub enum ReadUserFileItemError {
    EnsureUserStorageExists(#[from] EnsureUserStorageExistsError),
    ReadStorageItem(#[from] ReadStorageItemError),
    ReadDirContents(#[from] ReadDirContentsError),
    StoragePath(#[from] StoragePathError),
}

pub async fn read_user_file_item(
    storage_root_dir: &Path,
    path: &str,
    user: &User,
) -> Result<UserFilesHomeResult, ReadUserFileItemError> {
    let user_storage = UserStorage {
        user_id: user.id,
        storage_root: storage_root_dir.to_path_buf(),
    };

    ensure_user_storage_exists(&user_storage).await?;

    let path = StoragePath::new(user_storage.clone(), PathBuf::from(path))?;
    let storage_item = read_storage_item(&path).await?;
    let breadcrumb_segments = build_breadcrumb_segments(&user.username, &path.scoped_path);

    match storage_item.kind {
        StorageItemKind::Directory => {
            let mut dir_contents = read_dir_contents(&path).await?;

            dir_contents.sort_by_key(|item| item.path.path());
            dir_contents.sort_by_key(|item| item.kind.clone());

            let is_root_directory = path.scoped_path.file_name().is_none();
            let directory_name = path
                .scoped_path
                .file_name()
                .map(|directory_name| directory_name.to_string_lossy().to_string())
                .unwrap_or_else(|| user_root_directory_name(&user.username));

            Ok(UserFilesHomeResult::Directory(
                UserDirectoryContentsResult {
                    dir_contents,
                    path,
                    directory_name,
                    is_root_directory,
                    breadcrumb_segments,
                },
            ))
        }
        StorageItemKind::File => {
            let file_name = path
                .scoped_path
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_default();

            Ok(UserFilesHomeResult::File(UserFileItemResult {
                size: storage_item.size,
                path,
                file_name,
                breadcrumb_segments,
            }))
        }
    }
}

fn build_breadcrumb_segments(
    user_name: &str,
    scoped_path: &Path,
) -> Vec<DirectoryBreadcrumbSegment> {
    let mut segments = vec![DirectoryBreadcrumbSegment {
        name: user_root_directory_name(user_name),
        path: "/".to_owned(),
    }];

    let mut cumulative_path = PathBuf::from("/");

    for component in scoped_path.components() {
        if let std::path::Component::Normal(component_name) = component {
            cumulative_path.push(component_name);

            segments.push(DirectoryBreadcrumbSegment {
                name: component_name.to_string_lossy().to_string(),
                path: cumulative_path.to_string_lossy().to_string(),
            });
        }
    }

    // last item is current
    segments.pop();

    segments
}
