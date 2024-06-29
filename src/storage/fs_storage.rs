use std::{fs::File, io::Read, path::PathBuf};

use tokio::fs::DirEntry;

use super::{DirItem, FileItem, StorageError, StorageItem, StorageItemPath};

pub struct Storage;

#[async_recursion::async_recursion]
pub async fn dir_items(dir: PathBuf, recursive: bool) -> Result<Vec<DirEntry>, std::io::Error> {
    let mut dir_entries = tokio::fs::read_dir(&dir).await?;

    let mut result = Vec::new();

    while let Some(dir_entry) = dir_entries.next_entry().await? {
        if recursive && dir_entry.file_type().await?.is_dir() {
            result.extend(dir_items(dir_entry.path(), true).await?);
        }

        result.push(dir_entry);
    }

    Ok(result)
}

impl Storage {
    pub fn create() -> Self {
        Self
    }

    pub async fn create_directory(&self, path: &StorageItemPath) -> Result<DirItem, StorageError> {
        tokio::fs::create_dir_all(path.data_directory())
            .await
            .map_err(|error| StorageError::DirectoryCreation {
                source: error,
                path: path.clone(),
            })?;

        DirItem::from_metadata(
            path.clone(),
            tokio::fs::metadata(path.data_directory())
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    pub async fn storage_item(&self, path: &StorageItemPath) -> Result<StorageItem, StorageError> {
        StorageItem::from_metadata(
            path.clone(),
            tokio::fs::metadata(path.data_directory())
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    pub async fn list_storage_items(
        &self,
        path: &StorageItemPath,
    ) -> Result<Vec<StorageItem>, StorageError> {
        let mut dir_entries = tokio::fs::read_dir(path.data_directory())
            .await
            .map_err(StorageError::DirReader)?;

        let mut result = Vec::new();
        while let Some(dir_entry) = dir_entries
            .next_entry()
            .await
            .map_err(StorageError::DirReader)?
        {
            let dir_entry_path = path.storage.strip_data_dir(dir_entry.path());
            let path = StorageItemPath::new(path.storage.clone(), dir_entry_path)
                .map_err(StorageError::StorageItemPathCreation)?;

            result.push(
                StorageItem::from_dir_entry(path, dir_entry)
                    .await
                    .map_err(StorageError::MetadataReader)?,
            );
        }

        Ok(result)
    }

    pub async fn list_storage_items_recursively(
        &self,
        path: &StorageItemPath,
    ) -> Result<Vec<StorageItem>, StorageError> {
        let mut storage_items = Vec::new();

        for entry in dir_items(path.data_directory(), true)
            .await
            .map_err(StorageError::DirReader)?
        {
            let dir_entry_path_part = path.storage.strip_data_dir(entry.path());
            let dir_entry_path = StorageItemPath::new(path.storage.clone(), dir_entry_path_part)
                .map_err(StorageError::StorageItemPathCreation)?;

            let storage_item = StorageItem::from_dir_entry(dir_entry_path.clone(), entry)
                .await
                .map_err(StorageError::MetadataReader)?;

            storage_items.push(storage_item);
        }

        Ok(storage_items)
    }

    pub async fn add_file(
        &self,
        path: &StorageItemPath,
        mut file: File,
    ) -> Result<FileItem, StorageError> {
        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .map_err(|error| StorageError::FileReader {
                source: error,
                file_path: path.clone(),
            })?;

        tokio::fs::write(path.data_directory(), data)
            .await
            .map_err(|error| StorageError::FileWriter {
                source: error,
                file_path: path.clone(),
            })?;

        FileItem::from_metadata(
            path.clone(),
            tokio::fs::metadata(path.data_directory())
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    pub async fn move_item(
        &self,
        path: &StorageItemPath,
        new_path: &StorageItemPath,
    ) -> Result<StorageItem, StorageError> {
        tokio::fs::rename(&path.data_directory(), &new_path.data_directory())
            .await
            .map_err(StorageError::DirReader)?;

        StorageItem::from_metadata(
            new_path.clone(),
            tokio::fs::metadata(new_path.data_directory())
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    pub async fn copy_file(
        &self,
        path: &StorageItemPath,
        new_path: &StorageItemPath,
    ) -> Result<FileItem, StorageError> {
        tokio::fs::copy(&path.data_directory(), &new_path.data_directory())
            .await
            .map_err(|error| StorageError::FileWriter {
                source: error,
                file_path: new_path.clone(),
            })?;

        FileItem::from_metadata(
            new_path.clone(),
            tokio::fs::metadata(new_path.data_directory())
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    pub async fn copy_directory(
        &self,
        from_path: &StorageItemPath,
        to_path: &StorageItemPath,
    ) -> Result<DirItem, StorageError> {
        for entry in dir_items(from_path.data_directory(), true)
            .await
            .map_err(StorageError::DirReader)?
        {
            let dir_entry_path_part = from_path.strip_data_dir(entry.path());
            let mut to_path_dir_entry = to_path.scoped_path.clone();
            to_path_dir_entry.push(dir_entry_path_part.clone());

            let to_dir_entry_path =
                StorageItemPath::new(to_path.storage.clone(), to_path_dir_entry)
                    .map_err(StorageError::StorageItemPathCreation)?;

            let file_type = entry.file_type().await.map_err(StorageError::DirReader)?;

            if file_type.is_dir() {
                self.create_directory(&to_dir_entry_path).await?;
            } else if file_type.is_file() {
                let mut from_path_dir_entry = from_path.scoped_path.clone();
                from_path_dir_entry.push(dir_entry_path_part);

                let from_dir_entry_path =
                    StorageItemPath::new(from_path.storage.clone(), from_path_dir_entry)
                        .map_err(StorageError::StorageItemPathCreation)?;

                self.copy_file(&from_dir_entry_path, &to_dir_entry_path)
                    .await?;
            }
        }

        DirItem::from_metadata(
            to_path.clone(),
            tokio::fs::metadata(to_path.data_directory())
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    pub async fn remove_directory(&self, path: &StorageItemPath) -> Result<(), StorageError> {
        tokio::fs::remove_dir_all(path.data_directory())
            .await
            .map_err(StorageError::DirReader)
    }

    pub async fn remove_file(&self, path: &StorageItemPath) -> Result<(), StorageError> {
        tokio::fs::remove_file(path.data_directory())
            .await
            .map_err(|error| StorageError::FileReader {
                source: error,
                file_path: path.clone(),
            })
    }
}
