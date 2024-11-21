use std::path::PathBuf;

use bitsync_database::entity::User;
use bitsync_storage::{
    operation::{
        read::{read_dir_contents, read_storage_item, ReadDirContentsError, ReadStorageItemError},
        write::{
            ensure_user_storage_exists, write_file_stream, EnsureUserStorageExistsError,
            WriteFileStreamError,
        },
    },
    storage_item::StorageItem,
    storage_path::{StoragePath, StoragePathError},
    user_storage::UserStorage,
};
use futures::TryStreamExt;
use tokio_util::io::StreamReader;

pub struct UserFileUpload<S, B, E>
where
    S: futures::Stream<Item = Result<B, E>>,
    B: bytes::Buf,
    E: Into<std::io::Error>,
{
    pub file_name: String,
    pub stream: StreamReader<S, B>,
}

pub struct UserFileResult {
    pub dir_contents: Vec<StorageItem>,
}

#[derive(thiserror::Error, Debug)]
#[error("An error occurred during user file upload")]
pub enum UserFileUploadError {
    StorageEnsurance(#[from] EnsureUserStorageExistsError),
    WriteFileStream(#[from] WriteFileStreamError),
    StoragePath(#[from] StoragePathError),
    ReadDirContents(#[from] ReadDirContentsError),
}

pub async fn upload_user_file<S, B, E>(
    storage_root_dir: &PathBuf,
    path: &str,
    user: &User,
    file_name: &str,
    file_upload_stream: S,
) -> Result<UserFileResult, UserFileUploadError>
where
    S: futures::Stream<Item = Result<B, E>>,
    B: bytes::Buf,
    E: std::error::Error + Send + Sync + 'static,
{
    let user_storage = UserStorage {
        user_id: user.id,
        storage_root: storage_root_dir.clone(),
    };

    ensure_user_storage_exists(&user_storage).await?;

    let mut scoped_destination_path = PathBuf::from(path);
    scoped_destination_path.push(file_name);

    let destination_storage_path = StoragePath::new(user_storage.clone(), scoped_destination_path)?;

    let file_upload_stream_with_io_error =
        file_upload_stream.map_err(|error| std::io::Error::other(error));
    let file_upload_stream_reader = StreamReader::new(file_upload_stream_with_io_error);

    write_file_stream(&destination_storage_path, file_upload_stream_reader).await?;

    let directory_storage_path = StoragePath::new(user_storage.clone(), PathBuf::from(path))?;
    let mut dir_contents = read_dir_contents(&directory_storage_path).await?;

    dir_contents.sort_by_key(|item| item.path.path());
    dir_contents.sort_by_key(|item| item.kind.clone());

    Ok(UserFileResult { dir_contents })
}
