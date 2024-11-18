use std::{io::Error as IoError, path::PathBuf};

use crate::storage_path::StoragePath;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StorageItemKind {
    Directory,
    File,
}

#[non_exhaustive]
pub struct StorageItem {
    pub path: StoragePath,
    pub size: u64,
    pub kind: StorageItemKind,
}

#[derive(thiserror::Error, Debug)]
pub enum StorageItemCreationError {
    #[error("Storage item is of unsupported type: symlink")]
    IsSymlink { path: PathBuf },
    #[error("Could not gather metadata while creating a storage item")]
    Metadata { source: IoError, path: PathBuf },
}

impl StorageItem {
    pub(crate) async fn from_dir_entry(
        path: StoragePath,
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

impl TryFrom<(StoragePath, std::fs::Metadata)> for StorageItem {
    type Error = StorageItemCreationError;

    fn try_from(
        (path, metadata): (StoragePath, std::fs::Metadata),
    ) -> Result<Self, StorageItemCreationError> {
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
