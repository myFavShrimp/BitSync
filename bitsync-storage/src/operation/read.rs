use crate::{
    async_file_read::AsyncFileRead,
    storage_item::{StorageItem, StorageItemCreationError},
    storage_path::StoragePath,
};

use super::error::{MetadataError, OpenFileError, ReadDirectoryError};

#[derive(thiserror::Error, Debug)]
#[error("Could not read a directory's contents")]
pub enum ReadDirContentsError {
    ReadDirectory(#[from] ReadDirectoryError),
    StorageItemCreation(#[from] StorageItemCreationError),
}

pub async fn read_dir_contents(
    path: &StoragePath,
) -> Result<Vec<StorageItem>, ReadDirContentsError> {
    let mut dir_entries = tokio::fs::read_dir(path.local_directory())
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
        let scoped_path = path.storage.strip_data_dir(dir_entry.path());
        let storage_path = StoragePath {
            storage: path.storage.clone(),
            scoped_path,
        };

        storage_items.push(StorageItem::from_dir_entry(storage_path, dir_entry).await?);
    }

    Ok(storage_items)
}

#[derive(thiserror::Error, Debug)]
#[error("Could not read a file's contents")]
pub enum ReadFileStreamError {
    OpenFile(#[from] OpenFileError),
    StorageItemCreation(#[from] StorageItemCreationError),
}

pub async fn read_file_stream(path: &StoragePath) -> Result<AsyncFileRead, ReadFileStreamError> {
    let file = tokio::fs::File::open(path.local_directory())
        .await
        .map_err(|error| OpenFileError {
            source: error,
            path: path.local_directory(),
        })?;

    Ok(AsyncFileRead(file))
}

#[derive(thiserror::Error, Debug)]
#[error("Could not read an items information")]
pub enum ReadStorageItemError {
    Metadata(#[from] MetadataError),
    StorageItemCreation(#[from] StorageItemCreationError),
}

pub async fn read_storage_item(path: &StoragePath) -> Result<StorageItem, ReadStorageItemError> {
    let metadata = tokio::fs::metadata(&path.local_directory())
        .await
        .map_err(|error| MetadataError {
            source: error,
            path: path.local_directory(),
        })?;

    Ok(StorageItem::try_from((path.clone(), metadata))?)
}
