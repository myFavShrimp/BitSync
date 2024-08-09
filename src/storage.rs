use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use error::{DirectoryCreationError, ReadDirectoryError, StorageItemCreationError};

use crate::database::user::User;

mod error;

static USER_DATA_DIR: &str = "user";

#[derive(Clone, Debug)]
pub struct UserStorage {
    pub user: User,
    pub storage_root: PathBuf,
}

impl UserStorage {
    pub fn data_directory(&self) -> PathBuf {
        let mut storage_path = self.storage_root.clone();

        storage_path.push(USER_DATA_DIR);
        storage_path.push(self.user.id.to_string());

        storage_path
    }

    pub fn strip_data_dir(&self, path: PathBuf) -> PathBuf {
        path.strip_prefix(self.data_directory())
            .map(|path| path.to_path_buf())
            .unwrap_or(path)
    }
}

#[derive(Clone, Debug)]
pub struct StorageItemPath {
    storage: UserStorage,
    pub scoped_path: PathBuf,
}

impl StorageItemPath {
    pub fn new(storage: UserStorage, scoped_path: PathBuf) -> Self {
        let mut scoped_root = PathBuf::from("/");
        scoped_root.push(scoped_path);

        Self {
            storage,
            scoped_path: scoped_root,
        }
    }

    pub fn local_directory(&self) -> PathBuf {
        let scoped_path = self.scoped_path.clone();

        let mut user_directory = self.storage.data_directory();
        user_directory.push(scoped_path.strip_prefix("/").unwrap_or(&scoped_path));

        user_directory
    }

    pub fn push<P: AsRef<Path>>(&mut self, path: P) {
        self.scoped_path.push(path);
    }

    pub fn strip_data_dir(&self, path: PathBuf) -> PathBuf {
        path.strip_prefix(self.local_directory())
            .map(|path| path.to_path_buf())
            .unwrap_or(path)
    }
}

impl Display for StorageItemPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.scoped_path.to_string_lossy())
    }
}

pub enum StorageItemKind {
    Directory,
    File,
}

pub struct StorageItem {
    path: StorageItemPath,
    pub size: u64,
    pub kind: StorageItemKind,
}

impl StorageItem {
    pub fn file_name(&self) -> String {
        match self.path.scoped_path.file_name() {
            Some(file_name) => file_name.to_string_lossy().to_string(),
            None => String::new(),
        }
    }
}

impl StorageItem {
    pub async fn from_dir_entry(
        path: StorageItemPath,
        dir_entry: tokio::fs::DirEntry,
    ) -> Result<Self, StorageItemCreationError> {
        let metadata =
            dir_entry
                .metadata()
                .await
                .map_err(|error| StorageItemCreationError::Metadata {
                    source: error,
                    path: path.local_directory(),
                })?;

        let kind = if metadata.file_type().is_dir() {
            StorageItemKind::Directory
        } else if metadata.file_type().is_file() {
            StorageItemKind::File
        } else {
            return Err(StorageItemCreationError::IsSymlink {
                path: path.local_directory(),
            });
        };

        Ok(Self {
            path,
            size: metadata.len(),
            kind,
        })
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Could not ensure that the storage exists")]
pub struct EnsureExistsError(#[from] DirectoryCreationError);

#[derive(thiserror::Error, Debug)]
#[error("Could not read a directories contents")]
pub enum DirContentsError {
    ReadDirectory(#[from] ReadDirectoryError),
    StorageItemCreation(#[from] StorageItemCreationError),
}

impl UserStorage {
    pub async fn ensure_exists(&self) -> Result<(), EnsureExistsError> {
        tokio::fs::create_dir_all(&self.storage_root)
            .await
            .map_err(|error| DirectoryCreationError {
                source: error,
                path: PathBuf::from(&self.storage_root),
            })?;

        Ok(())
    }

    pub async fn dir_contents(
        &self,
        path: &StorageItemPath,
    ) -> Result<Vec<StorageItem>, DirContentsError> {
        let mut dir_entries =
            tokio::fs::read_dir(path.local_directory())
                .await
                .map_err(|error| ReadDirectoryError {
                    source: error,
                    path: path.local_directory(),
                })?;

        let mut storage_items = Vec::new();

        while let Some(dir_entry) =
            dir_entries
                .next_entry()
                .await
                .map_err(|error| ReadDirectoryError {
                    source: error,
                    path: path.local_directory(),
                })?
        {
            let dir_entry_path = path.storage.strip_data_dir(dir_entry.path());
            let path = StorageItemPath::new(path.storage.clone(), dir_entry_path);

            storage_items.push(StorageItem::from_dir_entry(path, dir_entry).await?);
        }

        Ok(storage_items)
    }
}
