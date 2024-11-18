use std::{fmt::Display, path::PathBuf};

use crate::{user_storage::UserStorage, validation::validate_scoped_path};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StoragePath {
    pub storage: UserStorage,
    pub scoped_path: PathBuf,
}

#[derive(thiserror::Error, Debug)]
#[error("Could not map path to storage")]
pub enum StoragePathError {
    Invalid(#[from] crate::validation::ScopedPathValidationError),
}

impl StoragePath {
    pub fn new(storage: UserStorage, scoped_path: PathBuf) -> Result<Self, StoragePathError> {
        validate_scoped_path(&scoped_path)?;

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

    pub fn file_name(&self) -> String {
        match self.scoped_path.file_name() {
            Some(file_name) => file_name.to_string_lossy().to_string(),
            None => String::new(),
        }
    }

    pub fn path(&self) -> String {
        self.scoped_path.to_string_lossy().to_string()
    }
}

impl Display for StoragePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.scoped_path.to_string_lossy())
    }
}
