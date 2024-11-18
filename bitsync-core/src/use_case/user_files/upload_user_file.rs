use std::path::PathBuf;

use bitsync_database::entity::User;
use bitsync_storage::{
    operation::{
        read::ReadStorageItemError,
        write::{
            ensure_user_storage_exists, write_file_stream, EnsureUserStorageExistsError,
            WriteFileStreamError,
        },
    },
    storage_path::{StoragePath, StoragePathError},
    user_storage::UserStorage,
};
use futures::TryStreamExt;
use tokio_util::io::StreamReader;

use super::shared::AsyncStorageItemRead;

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
    pub file: AsyncStorageItemRead,
    pub mime: mime_guess::Mime,
    pub path: StoragePath,
}

#[derive(thiserror::Error, Debug)]
#[error("An error occurred during user file upload")]
pub enum UserFileUploadError {
    StorageEnsurance(#[from] EnsureUserStorageExistsError),
    WriteFileStream(#[from] WriteFileStreamError),
    StoragePath(#[from] StoragePathError),
    ReadStorageItem(#[from] ReadStorageItemError),
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

    let mut upload_path = PathBuf::from(path);
    upload_path.push(file_name);

    let path = StoragePath::new(user_storage.clone(), upload_path)?;

    let file_upload_stream_with_io_error =
        file_upload_stream.map_err(|error| std::io::Error::other(error));

    let file_upload_stream_reader = StreamReader::new(file_upload_stream_with_io_error);

    let _storage_item = write_file_stream(&path, file_upload_stream_reader).await?;

    panic!("success");

    // match storage_item.kind {
    //     crate::storage::StorageItemKind::File => {
    //         let mime = mime_guess::from_path(&path.scoped_path).first_or_octet_stream();
    //         let file = StorageBackend::read_file_stream(&path).await?;

    //         Ok(UserFileResult {
    //             file: AsyncStorageItemRead::File(file),
    //             mime,
    //             path,
    //         })
    //     }
    //     crate::storage::StorageItemKind::Directory => {
    //         let (write_stream, read_stream) = tokio::io::duplex(4096);

    //         tokio::spawn(async move {
    //             match directory_zipping::write_zipped_storage_item_to_stream(
    //                 write_stream,
    //                 &storage_item,
    //             )
    //             .await
    //             {
    //                 Ok(_) => {}
    //                 Err(_) => todo!(),
    //             };
    //         });

    //         let mut dir_path = path.scoped_path.clone();
    //         dir_path.set_extension("zip");

    //         let fake_zip_path = StorageItemPath::new(user_storage.clone(), PathBuf::from(dir_path));

    //         let mime = mime_guess::from_path(&fake_zip_path.scoped_path).first_or_octet_stream();

    //         Ok(UserFileResult {
    //             file: AsyncStorageItemRead::Directory(read_stream),
    //             mime,
    //             path: fake_zip_path,
    //         })
    //     }
    // }
}
