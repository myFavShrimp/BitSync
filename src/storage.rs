use std::{fs::File, io::Read, path::PathBuf};

use uuid::Uuid;

use crate::{
    dto::DirectoryEntry,
    validate::{sanitize_directory_path, validate_file_path, PathValidationError},
};

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
}

pub struct Storage {
    pub storage_root: PathBuf,
}

impl Storage {
    pub async fn list_files(&self, path: &str) -> Result<Vec<DirectoryEntry>, StorageError> {
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
                DirectoryEntry::from_dir_entry(&self.storage_root, dir_entry)
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
    ) -> Result<DirectoryEntry, StorageError> {
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

        Ok(DirectoryEntry::from_metadata(
            file_path.to_string_lossy(),
            tokio::fs::metadata(data_path)
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)?)
    }
}