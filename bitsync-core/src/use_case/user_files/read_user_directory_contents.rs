use std::path::{Path, PathBuf};

use bitsync_database::entity::User;
use bitsync_storage::{
    operation::{
        read::{ReadDirContentsError, read_dir_contents},
        write::{EnsureUserStorageExistsError, ensure_user_storage_exists},
    },
    storage_item::StorageItem,
    storage_path::{StoragePath, StoragePathError},
    user_storage::UserStorage,
};

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

#[derive(thiserror::Error, Debug)]
#[error("failed to read user directory contents")]
pub enum ReadUserDirectoryContentsError {
    EnsureUserStorageExists(#[from] EnsureUserStorageExistsError),
    ReadDirContents(#[from] ReadDirContentsError),
    StoragePath(#[from] StoragePathError),
}

pub async fn read_user_directory_contents(
    storage_root_dir: &Path,
    path: &str,
    user: &User,
) -> Result<UserDirectoryContentsResult, ReadUserDirectoryContentsError> {
    let user_storage = UserStorage {
        user_id: user.id,
        storage_root: storage_root_dir.to_path_buf(),
    };

    ensure_user_storage_exists(&user_storage).await?;

    let path = StoragePath::new(user_storage.clone(), PathBuf::from(path))?;
    let mut dir_contents = read_dir_contents(&path).await?;

    dir_contents.sort_by_key(|item| item.path.path());
    dir_contents.sort_by_key(|item| item.kind.clone());

    let breadcrumb_segments = build_breadcrumb_segments(&path.scoped_path);

    let is_root_directory = path.scoped_path.file_name().is_none();

    let directory_name = path
        .scoped_path
        .file_name()
        .map(|directory_name| directory_name.to_string_lossy().to_string())
        .unwrap_or_else(|| user_root_directory_name(&user.username));

    Ok(UserDirectoryContentsResult {
        dir_contents,
        path,
        directory_name,
        is_root_directory,
        breadcrumb_segments,
    })
}

fn user_root_directory_name(user_name: &str) -> String {
    if user_name.ends_with('s') {
        format!("{user_name}' Storage")
    } else {
        format!("{user_name}'s Storage")
    }
}

fn build_breadcrumb_segments(scoped_path: &Path) -> Vec<DirectoryBreadcrumbSegment> {
    let mut segments = vec![DirectoryBreadcrumbSegment {
        name: "Root".to_owned(),
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

    segments.pop();

    segments
}
