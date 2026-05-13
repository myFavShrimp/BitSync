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

pub struct UserDirectoryContentsResult {
    pub dir_contents: Vec<StorageItem>,
    pub path: StoragePath,
    pub directory_name: String,
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

    let directory_name = path
        .scoped_path
        .file_name()
        .map(|directory_name| directory_name.to_string_lossy().to_string())
        .unwrap_or(user_root_directory_name(&user.username));

    Ok(UserDirectoryContentsResult {
        dir_contents,
        path,
        directory_name,
    })
}

fn user_root_directory_name(user_name: &str) -> String {
    if user_name.ends_with('s') {
        format!("{user_name}' Storage")
    } else {
        format!("{user_name}'s Storage")
    }
}
