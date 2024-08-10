use std::{io::Error as IOError, path::PathBuf};

#[derive(thiserror::Error, Debug)]
#[error("Failed to create directory")]
pub struct DirectoryCreationError {
    pub source: IOError,
    pub path: PathBuf,
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to read directory")]
pub struct ReadDirectoryError {
    pub source: IOError,
    pub path: PathBuf,
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to open a file")]
pub struct OpenFileError {
    pub source: IOError,
    pub path: PathBuf,
}

#[derive(thiserror::Error, Debug)]
pub enum StorageItemCreationError {
    #[error("Storage item is of unsupported type: symlink")]
    IsSymlink { path: PathBuf },
    #[error("Could not gather metadata while creating a storage item")]
    Metadata { source: IOError, path: PathBuf },
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to read an items metadata")]
pub struct MetadataError {
    pub source: IOError,
    pub path: PathBuf,
}
