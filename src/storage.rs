use std::{
    fmt::Display,
    fs::Metadata,
    path::{Path, PathBuf, StripPrefixError},
    sync::Arc,
};

use time::OffsetDateTime;
use tokio::sync::Mutex;

use crate::{
    database::user::User,
    validate::{validate_file_path, PathValidationError},
};

pub use fs_storage::Storage;

mod fs_storage;

static USER_DATA_DIR: &str = "user";

#[derive(thiserror::Error, Debug)]
pub enum StorageItemPathError {
    #[error(transparent)]
    StripPrefix(#[from] StripPrefixError),
    #[error(transparent)]
    PathValidation(#[from] PathValidationError),
}

#[derive(Clone, Debug)]
pub struct UserStorage {
    pub user: User,
    pub storage_root: PathBuf,
}

impl UserStorage {
    pub fn data_directory(&self) -> PathBuf {
        let mut storage_path = self.storage_root.clone();

        storage_path.push(USER_DATA_DIR);
        storage_path.push(self.user.id.to_string());

        storage_path
    }

    pub fn strip_data_dir(&self, path: PathBuf) -> PathBuf {
        path.strip_prefix(self.data_directory())
            .map(|path| path.to_path_buf())
            .unwrap_or(path)
    }
}

#[derive(Clone, Debug)]
pub struct StorageItemPath {
    storage: UserStorage,
    pub scoped_path: PathBuf,
}

impl StorageItemPath {
    pub fn new(storage: UserStorage, scoped_path: PathBuf) -> Result<Self, StorageItemPathError> {
        validate_file_path(&scoped_path.to_string_lossy())?;

        let mut scoped_root = PathBuf::from("/");
        scoped_root.push(scoped_path);

        Ok(Self {
            storage,
            scoped_path: scoped_root,
        })
    }

    pub fn data_directory(&self) -> PathBuf {
        let scoped_path = self.scoped_path.clone();

        let mut user_directory = self.storage.data_directory();
        user_directory.push(scoped_path.strip_prefix("/").unwrap_or(&scoped_path));

        user_directory
    }

    pub fn push<P: AsRef<Path>>(&mut self, path: P) {
        self.scoped_path.push(path);
    }

    pub fn strip_data_dir(&self, path: PathBuf) -> PathBuf {
        path.strip_prefix(self.data_directory())
            .map(|path| path.to_path_buf())
            .unwrap_or(path)
    }
}

impl Display for StorageItemPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.scoped_path.to_string_lossy())
    }
}

#[derive(Clone, Debug)]
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

#[derive(Debug)]
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
    #[error(transparent)]
    StripPrefix(#[from] StripPrefixError),
}
