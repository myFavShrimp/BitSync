use std::{
    fs::{File, Metadata},
    io::Read,
    path::{Path, PathBuf},
    sync::Arc,
};

use time::OffsetDateTime;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::validate::{sanitize_directory_path, validate_file_path, PathValidationError};

#[derive(async_graphql::SimpleObject, Clone)]
pub struct FileItem {
    pub path: String,
    pub size: u64,
    pub updated_at: OffsetDateTime,
}

impl FileItem {
    pub fn from_metadata<P: AsRef<str>>(
        scoped_path: P,
        metadata: Metadata,
    ) -> Result<Self, std::io::Error> {
        Ok(Self {
            path: scoped_path.as_ref().to_string(),
            size: metadata.len(),
            updated_at: metadata.modified()?.into(),
        })
    }
}

pub struct DirItemContent {
    pub files: Vec<FileItem>,
    pub directories: Vec<DirItem>,
}

#[derive(Clone)]
pub struct DirItem {
    pub path: String,
    pub updated_at: OffsetDateTime,
    pub content: Arc<Mutex<Option<DirItemContent>>>,
}

impl DirItem {
    pub fn from_metadata<P: AsRef<str>>(
        scoped_path: P,
        metadata: Metadata,
    ) -> Result<Self, std::io::Error> {
        let path = match scoped_path.as_ref() {
            path if path.is_empty() => String::from("/"),
            path => path.to_string(),
        };

        Ok(Self {
            path,
            updated_at: metadata.modified()?.into(),
            content: Arc::new(Mutex::new(None)),
        })
    }
}

pub enum StorageItem {
    DirItem(DirItem),
    FileItem(FileItem),
}

impl StorageItem {
    pub fn from_metadata<P: AsRef<str>>(
        scoped_path: P,
        metadata: Metadata,
    ) -> Result<Self, std::io::Error> {
        if metadata.is_dir() {
            Ok(Self::DirItem(DirItem::from_metadata(
                scoped_path,
                metadata,
            )?))
        } else {
            Ok(Self::FileItem(FileItem::from_metadata(
                scoped_path,
                metadata,
            )?))
        }
    }
    pub async fn from_dir_entry(
        storage_root: &Path,
        value: tokio::fs::DirEntry,
    ) -> Result<Self, std::io::Error> {
        let metadata = value.metadata().await?;
        let path = value.path().to_string_lossy().to_string();
        let path = path
            .strip_prefix(&storage_root.to_string_lossy().to_string())
            .unwrap_or(&path)
            .to_string();

        Self::from_metadata(path, metadata)
    }
}

static USER_DATA_DIR: &str = "user";

pub fn user_data_directory(mut storage_root: PathBuf, user_id: &Uuid) -> PathBuf {
    storage_root.push(USER_DATA_DIR);
    storage_root.push(user_id.to_string());

    storage_root
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error(transparent)]
    InvalidPath(#[from] PathValidationError),
    #[error(transparent)]
    DirReader(std::io::Error),
    #[error(transparent)]
    MetadataReader(std::io::Error),
    #[error("Could not read the data of '{file_path}'")]
    FileReader {
        #[source]
        source: std::io::Error,
        file_path: String,
    },
    #[error("Could not write the data of '{file_path}'")]
    FileWriter {
        #[source]
        source: std::io::Error,
        file_path: String,
    },
    #[error("Could not create the directory '{path}'")]
    DirectoryCreation {
        #[source]
        source: std::io::Error,
        path: String,
    },
}

pub struct Storage {
    pub storage_root: PathBuf,
}

impl Storage {
    pub async fn create_directory(&self, path: &str) -> Result<DirItem, StorageError> {
        let path = sanitize_directory_path(path);
        validate_file_path(path)?;

        let mut data_path = self.storage_root.clone();
        data_path.push(path);

        tokio::fs::create_dir_all(&data_path)
            .await
            .map_err(|error| StorageError::DirectoryCreation {
                source: error,
                path: path.to_string(),
            })?;

        DirItem::from_metadata(
            path,
            tokio::fs::metadata(data_path)
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    pub async fn storage_item(&self, path: &str) -> Result<StorageItem, StorageError> {
        let path = sanitize_directory_path(path);
        validate_file_path(path)?;

        let mut data_path = self.storage_root.clone();
        data_path.push(path);

        StorageItem::from_metadata(
            path,
            tokio::fs::metadata(data_path)
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    pub async fn list_storage_items(&self, path: &str) -> Result<Vec<StorageItem>, StorageError> {
        let path = sanitize_directory_path(path);
        validate_file_path(path)?;

        let mut data_path = self.storage_root.clone();
        data_path.push(path);

        let mut dir_entries = tokio::fs::read_dir(data_path)
            .await
            .map_err(StorageError::DirReader)?;

        let mut result = Vec::new();
        while let Some(dir_entry) = dir_entries
            .next_entry()
            .await
            .map_err(StorageError::DirReader)?
        {
            result.push(
                StorageItem::from_dir_entry(&self.storage_root, dir_entry)
                    .await
                    .map_err(StorageError::MetadataReader)?,
            );
        }

        Ok(result)
    }

    pub async fn add_file(
        &self,
        path: &str,
        file_name: &str,
        mut file: File,
    ) -> Result<FileItem, StorageError> {
        let path = sanitize_directory_path(path);
        validate_file_path(path)?;

        let mut data_path = self.storage_root.clone();
        data_path.push(path);
        data_path.push(file_name);

        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .map_err(|error| StorageError::FileReader {
                source: error,
                file_path: format!("{path}/{file_name}"),
            })?;

        tokio::fs::write(&data_path, data)
            .await
            .map_err(|error| StorageError::FileWriter {
                source: error,
                file_path: format!("{path}/{file_name}"),
            })?;

        let mut file_path = PathBuf::from(path);
        file_path.push(file_name);

        FileItem::from_metadata(
            file_path.to_string_lossy(),
            tokio::fs::metadata(data_path)
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    pub async fn move_item(&self, path: &str, new_path: &str) -> Result<FileItem, StorageError> {
        let path = sanitize_directory_path(path);
        validate_file_path(path)?;
        let new_path = sanitize_directory_path(new_path);
        validate_file_path(new_path)?;

        let mut data_path = self.storage_root.clone();
        data_path.push(path);
        let mut new_data_path = self.storage_root.clone();
        new_data_path.push(new_path);

        tokio::fs::rename(&data_path, &new_data_path)
            .await
            .map_err(StorageError::DirReader)?;

        FileItem::from_metadata(
            new_path,
            tokio::fs::metadata(new_data_path)
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }
}
