use std::{
    fmt::Display,
    fs::{File, Metadata},
    io::Read,
    path::{Path, PathBuf, StripPrefixError},
    sync::Arc,
};

use time::OffsetDateTime;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::validate::{validate_file_path, PathValidationError};

static USER_DATA_DIR: &str = "user";

fn user_data_directory(mut storage_root: PathBuf, user_id: &Uuid) -> PathBuf {
    storage_root.push(USER_DATA_DIR);
    storage_root.push(user_id.to_string());

    storage_root
}

#[derive(thiserror::Error, Debug)]
pub enum StorageItemPathError {
    #[error(transparent)]
    StripPrefix(#[from] StripPrefixError),
    #[error(transparent)]
    PathValidation(#[from] PathValidationError),
}

#[derive(Clone, Debug)]
pub struct StorageItemPath {
    storage_root: PathBuf,
    pub scoped_path: PathBuf,
    user_id: Uuid,
}

impl StorageItemPath {
    pub fn new(
        storage_root: PathBuf,
        scoped_path: PathBuf,
        user_id: Uuid,
    ) -> Result<Self, StorageItemPathError> {
        validate_file_path(&scoped_path.to_string_lossy())?;

        let mut scoped_root = PathBuf::from("/");
        scoped_root.push(scoped_path);

        Ok(Self {
            storage_root,
            scoped_path: scoped_root,
            user_id,
        })
    }

    pub fn system_data_directory(&self) -> PathBuf {
        let scoped_path = self.scoped_path.clone();

        let mut user_directory = user_data_directory(self.storage_root.clone(), &self.user_id);
        user_directory.push(scoped_path.strip_prefix("/").unwrap_or(&scoped_path));

        user_directory
    }

    pub fn push<P: AsRef<Path>>(&mut self, path: P) {
        self.scoped_path.push(path);
    }

    pub fn new_with_stripped_storage_root(
        &self,
        path: PathBuf,
    ) -> Result<Self, StorageItemPathError> {
        let user_directory = user_data_directory(self.storage_root.clone(), &self.user_id);

        let path = path
            .strip_prefix(user_directory)
            .map(|path| path.to_path_buf())?;

        Self::new(self.storage_root.clone(), path, self.user_id)
    }
}

impl Display for StorageItemPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.scoped_path.to_string_lossy())
    }
}

#[async_graphql::Scalar]
impl async_graphql::ScalarType for StorageItemPath {
    fn parse(_value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        unimplemented!("StorageItemPath GraphQL parsing");
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.scoped_path.to_string_lossy().to_string())
    }
}

#[derive(async_graphql::SimpleObject, Clone, Debug)]
pub struct FileItem {
    pub path: StorageItemPath,
    pub size: u64,
    pub updated_at: OffsetDateTime,
}

impl FileItem {
    pub fn from_metadata(
        path: StorageItemPath,
        metadata: Metadata,
    ) -> Result<Self, std::io::Error> {
        Ok(Self {
            path,
            size: metadata.len(),
            updated_at: metadata.modified()?.into(),
        })
    }
}

#[derive(Debug)]
pub struct DirItemContent {
    pub files: Vec<FileItem>,
    pub directories: Vec<DirItem>,
}

#[derive(Clone, Debug)]
pub struct DirItem {
    pub path: StorageItemPath,
    pub updated_at: OffsetDateTime,
    pub content: Arc<Mutex<Option<DirItemContent>>>,
}

impl DirItem {
    pub fn from_metadata(
        path: StorageItemPath,
        metadata: Metadata,
    ) -> Result<Self, std::io::Error> {
        Ok(Self {
            path,
            updated_at: metadata.modified()?.into(),
            content: Arc::new(Mutex::new(None)),
        })
    }
}

#[derive(async_graphql::Union, Debug)]
pub enum StorageItem {
    DirItem(DirItem),
    FileItem(FileItem),
}

impl StorageItem {
    pub fn path(&self) -> &StorageItemPath {
        match self {
            StorageItem::DirItem(dir_item) => &dir_item.path,
            StorageItem::FileItem(file_item) => &file_item.path,
        }
    }

    pub fn from_metadata(
        path: StorageItemPath,
        metadata: Metadata,
    ) -> Result<Self, std::io::Error> {
        if metadata.is_dir() {
            Ok(Self::DirItem(DirItem::from_metadata(path, metadata)?))
        } else {
            Ok(Self::FileItem(FileItem::from_metadata(path, metadata)?))
        }
    }
    pub async fn from_dir_entry(
        path: StorageItemPath,
        value: tokio::fs::DirEntry,
    ) -> Result<Self, std::io::Error> {
        let metadata = value.metadata().await?;

        Self::from_metadata(path, metadata)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error(transparent)]
    DirReader(std::io::Error),
    #[error(transparent)]
    MetadataReader(std::io::Error),
    #[error("Could not read the data of '{file_path}'")]
    FileReader {
        #[source]
        source: std::io::Error,
        file_path: StorageItemPath,
    },
    #[error("Could not write the data of '{file_path}'")]
    FileWriter {
        #[source]
        source: std::io::Error,
        file_path: StorageItemPath,
    },
    #[error("Could not create the directory '{path}'")]
    DirectoryCreation {
        #[source]
        source: std::io::Error,
        path: StorageItemPath,
    },
    #[error(transparent)]
    StorageItemPathCreation(#[from] StorageItemPathError),
}

pub struct Storage;

impl Storage {
    pub async fn create_directory(&self, path: &StorageItemPath) -> Result<DirItem, StorageError> {
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

    pub async fn storage_item(&self, path: &StorageItemPath) -> Result<StorageItem, StorageError> {
        StorageItem::from_metadata(
            path.clone(),
            tokio::fs::metadata(path.system_data_directory())
                .await
                .map_err(StorageError::MetadataReader)?,
        )
        .map_err(StorageError::MetadataReader)
    }

    pub async fn list_storage_items(
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

    pub async fn list_storage_items_recursively(
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

    pub async fn move_item(
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

    pub async fn remove_directory(&self, path: &StorageItemPath) -> Result<(), StorageError> {
        tokio::fs::remove_dir_all(path.system_data_directory())
            .await
            .map_err(StorageError::DirReader)
    }

    pub async fn remove_file(&self, path: &StorageItemPath) -> Result<(), StorageError> {
        tokio::fs::remove_file(path.system_data_directory())
            .await
            .map_err(|error| StorageError::FileReader {
                source: error,
                file_path: path.clone(),
            })
    }
}
