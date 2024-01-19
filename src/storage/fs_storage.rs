use std::{fs::File, io::Read};

use super::{DirItem, FileItem, Storage, StorageError, StorageItem, StorageItemPath};

pub struct FsStorage;

impl Storage for FsStorage {
    async fn create_directory(&self, path: &StorageItemPath) -> Result<DirItem, StorageError> {
        tokio::fs::create_dir_all(path.system_data_directory())
            .await
            .map_err(|error| StorageError::DirectoryCreation {
                source: error,
                path: path.clone(),
            })?;

        DirItem::from_metadata(
            path.clone(),
            tokio::fs::metadata(path.system_data_directory())
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    async fn storage_item(&self, path: &StorageItemPath) -> Result<StorageItem, StorageError> {
        StorageItem::from_metadata(
            path.clone(),
            tokio::fs::metadata(path.system_data_directory())
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    async fn list_storage_items(
        &self,
        path: &StorageItemPath,
    ) -> Result<Vec<StorageItem>, StorageError> {
        let mut dir_entries = tokio::fs::read_dir(path.system_data_directory())
            .await
            .map_err(StorageError::DirReader)?;

        let mut result = Vec::new();
        while let Some(dir_entry) = dir_entries
            .next_entry()
            .await
            .map_err(StorageError::DirReader)?
        {
            let path = path
                .new_with_stripped_storage_root(dir_entry.path())
                .map_err(StorageError::StorageItemPathCreation)?;
            result.push(
                StorageItem::from_dir_entry(path, dir_entry)
                    .await
                    .map_err(StorageError::MetadataReader)?,
            );
        }

        Ok(result)
    }

    async fn list_storage_items_recursively(
        &self,
        path: &StorageItemPath,
    ) -> Result<Vec<StorageItem>, StorageError> {
        let mut dirs_to_process = vec![tokio::fs::read_dir(path.system_data_directory())
            .await
            .map_err(StorageError::DirReader)?];

        let mut storage_items = Vec::new();

        'diriter: loop {
            dirs_to_process = {
                let mut new_dirs_to_process = Vec::new();

                for mut dir in dirs_to_process {
                    while let Some(dir_entry) =
                        dir.next_entry().await.map_err(StorageError::DirReader)?
                    {
                        if dir_entry
                            .file_type()
                            .await
                            .map_err(StorageError::DirReader)?
                            .is_dir()
                        {
                            new_dirs_to_process.push(
                                tokio::fs::read_dir(dir_entry.path())
                                    .await
                                    .map_err(StorageError::DirReader)?,
                            );
                        }

                        let path = path.new_with_stripped_storage_root(dir_entry.path())?;

                        let storage_item = StorageItem::from_dir_entry(path, dir_entry)
                            .await
                            .map_err(StorageError::MetadataReader)?;

                        storage_items.push(storage_item);
                    }
                }

                new_dirs_to_process
            };

            if dirs_to_process.is_empty() {
                break 'diriter;
            }
        }

        Ok(storage_items)
    }

    async fn add_file(
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

        tokio::fs::write(path.system_data_directory(), data)
            .await
            .map_err(|error| StorageError::FileWriter {
                source: error,
                file_path: path.clone(),
            })?;

        FileItem::from_metadata(
            path.clone(),
            tokio::fs::metadata(path.system_data_directory())
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    async fn move_item(
        &self,
        path: &StorageItemPath,
        new_path: &StorageItemPath,
    ) -> Result<StorageItem, StorageError> {
        tokio::fs::rename(
            &path.system_data_directory(),
            &new_path.system_data_directory(),
        )
        .await
        .map_err(StorageError::DirReader)?;

        StorageItem::from_metadata(
            new_path.clone(),
            tokio::fs::metadata(new_path.system_data_directory())
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    async fn copy_file(
        &self,
        path: &StorageItemPath,
        new_path: &StorageItemPath,
    ) -> Result<FileItem, StorageError> {
        tokio::fs::copy(
            &path.system_data_directory(),
            &new_path.system_data_directory(),
        )
        .await
        .map_err(|error| StorageError::FileWriter {
            source: error,
            file_path: new_path.clone(),
        })?;

        FileItem::from_metadata(
            new_path.clone(),
            tokio::fs::metadata(new_path.system_data_directory())
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    async fn copy_directory(
        &self,
        path: &StorageItemPath,
        new_path: &StorageItemPath,
    ) -> Result<DirItem, StorageError> {
        let mut dirs_to_process = vec![tokio::fs::read_dir(path.system_data_directory())
            .await
            .map_err(StorageError::DirReader)?];

        let mut storage_items = Vec::new();

        'diriter: loop {
            dirs_to_process = {
                let mut new_dirs_to_process = Vec::new();

                for mut dir in dirs_to_process {
                    while let Some(dir_entry) =
                        dir.next_entry().await.map_err(StorageError::DirReader)?
                    {
                        let file_type = dir_entry
                            .file_type()
                            .await
                            .map_err(StorageError::DirReader)?;

                        if file_type.is_dir() {
                            new_dirs_to_process.push(
                                tokio::fs::read_dir(dir_entry.path())
                                    .await
                                    .map_err(StorageError::DirReader)?,
                            );
                            // todo create dir
                            // tokio::fs::create_dir_all()
                        } else if file_type.is_file() {
                            let entry_path = dir_entry.path();
                            let entry_path_from_new_dir =
                                entry_path.strip_prefix(path.system_data_directory())?;

                            let mut new_file_path = new_path.scoped_path.clone();
                            new_file_path.push(entry_path_from_new_dir);

                            let new_path = path.new_with_stripped_storage_root(new_file_path)?;

                            tokio::fs::copy(
                                path.system_data_directory(),
                                new_path.system_data_directory(),
                            )
                            .await
                            .map_err(|error| {
                                StorageError::FileWriter {
                                    source: error,
                                    file_path: new_path.clone(),
                                }
                            })?;

                            new_dirs_to_process.push(
                                tokio::fs::read_dir(dir_entry.path())
                                    .await
                                    .map_err(StorageError::DirReader)?,
                            );
                        }

                        let path = path.new_with_stripped_storage_root(dir_entry.path())?;

                        // tokio::
                        let storage_item = StorageItem::from_dir_entry(path, dir_entry)
                            .await
                            .map_err(StorageError::MetadataReader)?;

                        storage_items.push(storage_item);
                    }
                }

                new_dirs_to_process
            };

            if dirs_to_process.is_empty() {
                break 'diriter;
            }
        }

        DirItem::from_metadata(
            new_path.clone(),
            tokio::fs::metadata(new_path.system_data_directory())
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    async fn remove_directory(&self, path: &StorageItemPath) -> Result<(), StorageError> {
        tokio::fs::remove_dir_all(path.system_data_directory())
            .await
            .map_err(StorageError::DirReader)
    }

    async fn remove_file(&self, path: &StorageItemPath) -> Result<(), StorageError> {
        tokio::fs::remove_file(path.system_data_directory())
            .await
            .map_err(|error| StorageError::FileReader {
                source: error,
                file_path: path.clone(),
            })
    }
}
