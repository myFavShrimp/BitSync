use std::{
    fmt::Display,
    path::{Path, PathBuf, StripPrefixError},
};

use crate::{
    database::user::User,
    validate::{validate_file_path, PathValidationError},
};

mod error;

static USER_DATA_DIR: &str = "user";

#[derive(thiserror::Error, Debug)]
pub enum StorageItemPathError {
    #[error(transparent)]
    StripPrefix(#[from] StripPrefixError),
    #[error(transparent)]
    PathValidation(#[from] PathValidationError),
}

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
    pub fn new(storage: UserStorage, scoped_path: PathBuf) -> Result<Self, StorageItemPathError> {
        validate_file_path(&scoped_path.to_string_lossy())?;

        let mut scoped_root = PathBuf::from("/");
        scoped_root.push(scoped_path);

        Ok(Self {
            storage,
            scoped_path: scoped_root,
        })
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

#[derive(thiserror::Error, Debug)]
pub enum StorageItemError {
    #[error("Storage item is of unsupported type: symlink")]
    IsSymlink,
    #[error("Could not gather a storage item's metadata")]
    Metadata(#[from] std::io::Error),
}

impl StorageItem {
    pub async fn from_dir_entry(
        path: StorageItemPath,
        dir_entry: tokio::fs::DirEntry,
    ) -> Result<Self, StorageItemError> {
        let metadata = dir_entry.metadata().await?;

        let kind = if metadata.file_type().is_dir() {
            StorageItemKind::Directory
        } else if metadata.file_type().is_file() {
            StorageItemKind::File
        } else {
            return Err(StorageItemError::IsSymlink);
        };

        Ok(Self {
            path,
            size: metadata.len(),
            kind,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Failed to read directory")]
    ReadDir(#[from] std::io::Error),
    #[error("Failed to create scoped storage path data")]
    StorageItemPathCreation(#[from] StorageItemPathError),
    #[error("Failed to read storage item data")]
    StorageItemCreation(#[from] StorageItemError),
    #[error("Failed to create directory")]
    DirectoryCreation {
        source: std::io::Error,
        path: std::path::PathBuf,
    },
}

impl UserStorage {
    pub async fn ensure_exists(&self) -> Result<(), StorageError> {
        tokio::fs::create_dir_all(&self.storage_root)
            .await
            .map_err(|error| StorageError::DirectoryCreation {
                source: error,
                path: PathBuf::from(&self.storage_root),
            })?;

        Ok(())
    }

    pub async fn dir_contents(
        &self,
        path: &StorageItemPath,
    ) -> Result<Vec<StorageItem>, StorageError> {
        let mut dir_entries = tokio::fs::read_dir(path.local_directory())
            .await
            .map_err(StorageError::ReadDir)?;

        let mut storage_items = Vec::new();

        while let Some(dir_entry) = dir_entries
            .next_entry()
            .await
            .map_err(StorageError::ReadDir)?
        {
            let dir_entry_path = path.storage.strip_data_dir(dir_entry.path());
            let path = StorageItemPath::new(path.storage.clone(), dir_entry_path)
                .map_err(StorageError::StorageItemPathCreation)?;

            storage_items.push(StorageItem::from_dir_entry(path, dir_entry).await?);
        }

        Ok(storage_items)
    }
}
