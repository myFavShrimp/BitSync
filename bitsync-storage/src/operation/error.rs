use std::{io::Error as IoError, path::PathBuf};

// -----

#[derive(thiserror::Error, Debug)]
#[error("Failed to create directory")]
pub struct DirectoryCreationError {
    pub source: IoError,
    pub path: PathBuf,
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to read directory")]
pub struct ReadDirectoryError {
    pub source: IoError,
    pub path: PathBuf,
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to open a file")]
pub struct OpenFileError {
    pub source: IoError,
    pub path: PathBuf,
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to read an items metadata")]
pub struct MetadataError {
    pub source: IoError,
    pub path: PathBuf,
}
