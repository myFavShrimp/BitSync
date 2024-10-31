use std::{
    fmt::Display,
    path::{Path, PathBuf},
    pin::Pin,
};

use error::{
    DirectoryCreationError, MetadataError, OpenFileError, ReadDirectoryError, RemoveDirectoryError,
    RemoveFileError, StorageItemCreationError,
};
use futures::pin_mut;
use tokio::io::BufWriter;
use tokio_util::io::StreamReader;

use crate::database::user::User;

pub mod error;

static USER_DATA_DIR: &str = "user";

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
    pub fn new(storage: UserStorage, scoped_path: PathBuf) -> Self {
        let mut scoped_root = PathBuf::from("/");
        scoped_root.push(scoped_path);

        Self {
            storage,
            scoped_path: scoped_root,
        }
    }

    pub fn local_directory(&self) -> PathBuf {
        let scoped_path = self.scoped_path.clone();

        let mut user_directory = self.storage.data_directory();
        user_directory.push(scoped_path.strip_prefix("/").unwrap_or(&scoped_path));

        user_directory
    }

    pub fn push<P: AsRef<Path>>(&mut self, path: P) {
        self.scoped_path.push(path);
    }

    pub fn strip_data_dir(&self, path: PathBuf) -> PathBuf {
        path.strip_prefix(self.local_directory())
            .map(|path| path.to_path_buf())
            .unwrap_or(path)
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

impl Display for StorageItemPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.scoped_path.to_string_lossy())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StorageItemKind {
    Directory,
    File,
}

pub struct StorageItem {
    pub path: StorageItemPath,
    pub size: u64,
    pub kind: StorageItemKind,
}

impl StorageItem {
    pub async fn from_dir_entry(
        path: StorageItemPath,
        dir_entry: tokio::fs::DirEntry,
    ) -> Result<Self, StorageItemCreationError> {
        let metadata =
            dir_entry
                .metadata()
                .await
                .map_err(|error| StorageItemCreationError::Metadata {
                    source: error,
                    path: path.local_directory(),
                })?;

        let kind = if metadata.file_type().is_dir() {
            StorageItemKind::Directory
        } else if metadata.file_type().is_file() {
            StorageItemKind::File
        } else {
            return Err(StorageItemCreationError::IsSymlink {
                path: path.local_directory(),
            });
        };

        Ok(Self {
            path,
            size: metadata.len(),
            kind,
        })
    }
}

impl TryFrom<(StorageItemPath, std::fs::Metadata)> for StorageItem {
    type Error = StorageItemCreationError;

    fn try_from(
        (path, metadata): (StorageItemPath, std::fs::Metadata),
    ) -> Result<Self, StorageItemCreationError> {
        let kind = if metadata.file_type().is_dir() {
            StorageItemKind::Directory
        } else if metadata.file_type().is_file() {
            StorageItemKind::File
        } else {
            return Err(StorageItemCreationError::IsSymlink {
                path: path.local_directory(),
            });
        };

        Ok(Self {
            path,
            size: metadata.len(),
            kind,
        })
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Could not ensure that the storage exists")]
pub struct EnsureExistsError(#[from] DirectoryCreationError);

#[derive(thiserror::Error, Debug)]
#[error("Could not read a directory's contents")]
pub enum DirContentsError {
    ReadDirectory(#[from] ReadDirectoryError),
    StorageItemCreation(#[from] StorageItemCreationError),
}

pub struct AsyncFileRead(tokio::fs::File);

impl tokio::io::AsyncRead for AsyncFileRead {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let self_mut = self.get_mut();
        let inner = Pin::new(&mut self_mut.0);

        inner.poll_read(cx, buf)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Could not read a file's contents")]
pub enum FileWriteError {
    OpenFile(#[from] OpenFileError),
    StorageItemCreation(#[from] StorageItemCreationError),
    StreamWrite(#[source] std::io::Error),
}

#[derive(thiserror::Error, Debug)]
#[error("Could not read a file's contents")]
pub enum FileContentsError {
    OpenFile(#[from] OpenFileError),
    StorageItemCreation(#[from] StorageItemCreationError),
}

#[derive(thiserror::Error, Debug)]
#[error("Could not read an items information")]
pub enum StorageItemError {
    Metadata(#[from] MetadataError),
    StorageItemCreation(#[from] StorageItemCreationError),
}

pub struct StorageBackend;

impl StorageBackend {
    pub async fn ensure_exists(storage: &UserStorage) -> Result<(), EnsureExistsError> {
        tokio::fs::create_dir_all(&storage.storage_root)
            .await
            .map_err(|error| DirectoryCreationError {
                source: error,
                path: storage.storage_root.clone(),
            })?;

        Ok(())
    }

    pub async fn dir_contents(
        path: &StorageItemPath,
    ) -> Result<Vec<StorageItem>, DirContentsError> {
        let mut dir_entries =
            tokio::fs::read_dir(path.local_directory())
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
            let dir_entry_path = path.storage.strip_data_dir(dir_entry.path());
            let path = StorageItemPath::new(path.storage.clone(), dir_entry_path);

            storage_items.push(StorageItem::from_dir_entry(path, dir_entry).await?);
        }

        Ok(storage_items)
    }

    pub async fn read_file_stream(
        path: &StorageItemPath,
    ) -> Result<AsyncFileRead, FileContentsError> {
        let file = tokio::fs::File::open(path.local_directory())
            .await
            .map_err(|error| OpenFileError {
                source: error,
                path: path.local_directory(),
            })?;

        Ok(AsyncFileRead(file))
    }

    pub async fn write_file_stream<S, B, E>(
        path: &StorageItemPath,
        stream: StreamReader<S, B>,
    ) -> Result<AsyncFileRead, FileWriteError>
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
            .map_err(FileWriteError::StreamWrite)?;

        let file = tokio::fs::File::open(path.local_directory())
            .await
            .map_err(|error| OpenFileError {
                source: error,
                path: path.local_directory(),
            })?;

        Ok(AsyncFileRead(file))
    }

    pub async fn storage_item(path: &StorageItemPath) -> Result<StorageItem, StorageItemError> {
        let metadata = tokio::fs::metadata(&path.local_directory())
            .await
            .map_err(|error| MetadataError {
                source: error,
                path: path.local_directory(),
            })?;

        Ok(StorageItem::try_from((path.clone(), metadata))?)
    }

    pub async fn delete_directory(path: &StorageItemPath) -> Result<(), RemoveDirectoryError> {
        tokio::fs::remove_dir_all(path.local_directory())
            .await
            .map_err(|error| RemoveDirectoryError {
                source: error,
                path: path.local_directory(),
            })?;

        Ok(())
    }

    pub async fn delete_file(path: &StorageItemPath) -> Result<(), RemoveFileError> {
        tokio::fs::remove_file(path.local_directory())
            .await
            .map_err(|error| RemoveFileError {
                source: error,
                path: path.local_directory(),
            })?;

        Ok(())
    }
}
