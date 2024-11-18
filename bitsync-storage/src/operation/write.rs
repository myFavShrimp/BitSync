use std::{io::Error as IoError, path::PathBuf};

use futures::pin_mut;
use tokio::io::BufWriter;
use tokio_util::io::StreamReader;

use crate::{
    async_file_read::AsyncFileRead, storage_item::StorageItemCreationError,
    storage_path::StoragePath, user_storage::UserStorage,
};

use super::error::{DirectoryCreationError, OpenFileError};

#[derive(thiserror::Error, Debug)]
#[error("Could not ensure that the storage exists")]
pub struct EnsureUserStorageExistsError(#[from] DirectoryCreationError);

pub async fn ensure_user_storage_exists(
    storage: &UserStorage,
) -> Result<(), EnsureUserStorageExistsError> {
    tokio::fs::create_dir_all(&storage.storage_root)
        .await
        .map_err(|error| DirectoryCreationError {
            source: error,
            path: storage.storage_root.clone(),
        })?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
#[error("Could not read a file's contents")]
pub enum AsyncFileReadError {
    OpenFile(#[from] OpenFileError),
    StorageItemCreation(#[from] StorageItemCreationError),
}

#[derive(thiserror::Error, Debug)]
#[error("Could not read a file's contents")]
pub enum WriteFileStreamError {
    OpenFile(#[from] OpenFileError),
    StorageItemCreation(#[from] StorageItemCreationError),
    StreamWrite(#[source] std::io::Error),
}

pub async fn write_file_stream<S, B, E>(
    path: &StoragePath,
    stream: StreamReader<S, B>,
) -> Result<AsyncFileRead, WriteFileStreamError>
where
    S: futures::Stream<Item = Result<B, E>>,
    B: bytes::Buf,
    E: Into<std::io::Error>,
{
    let file = tokio::fs::File::create(path.local_directory())
        .await
        .map_err(|error| OpenFileError {
            source: error,
            path: path.local_directory(),
        })?;

    let mut file_writer = BufWriter::new(file);
    pin_mut!(stream);

    tokio::io::copy(&mut stream, &mut file_writer)
        .await
        .map_err(WriteFileStreamError::StreamWrite)?;

    let file = tokio::fs::File::open(path.local_directory())
        .await
        .map_err(|error| OpenFileError {
            source: error,
            path: path.local_directory(),
        })?;

    Ok(AsyncFileRead(file))
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to remove directory")]
pub struct DeleteDirectoryError {
    pub source: IoError,
    pub path: PathBuf,
}

pub async fn delete_directory(path: &StoragePath) -> Result<(), DeleteDirectoryError> {
    tokio::fs::remove_dir_all(path.local_directory())
        .await
        .map_err(|error| DeleteDirectoryError {
            source: error,
            path: path.local_directory(),
        })?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to remove file")]
pub struct DeleteFileError {
    pub source: IoError,
    pub path: PathBuf,
}

pub async fn delete_file(path: &StoragePath) -> Result<(), DeleteFileError> {
    tokio::fs::remove_file(path.local_directory())
        .await
        .map_err(|error| DeleteFileError {
            source: error,
            path: path.local_directory(),
        })?;

    Ok(())
}
