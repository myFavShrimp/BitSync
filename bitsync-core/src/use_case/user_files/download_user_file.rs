use std::path::PathBuf;

use bitsync_database::entity::User;
use bitsync_storage::{
    operation::{
        read::{read_file_stream, read_storage_item, ReadFileStreamError, ReadStorageItemError},
        write::{ensure_user_storage_exists, EnsureUserStorageExistsError},
    },
    storage_item::StorageItemKind,
    storage_path::{StoragePath, StoragePathError},
    user_storage::UserStorage,
};
use tracing::Level;

use super::shared::AsyncStorageItemRead;

mod directory_zipping;

pub struct UserFileDownloadResult {
    pub file: AsyncStorageItemRead,
    pub mime: mime_guess::Mime,
    pub path: StoragePath,
}

#[derive(thiserror::Error, Debug)]
#[error("An error occurred during user file download")]
pub enum UserFileDownloadError {
    EnsureUserStorageExists(#[from] EnsureUserStorageExistsError),
    ReadFileStream(#[from] ReadFileStreamError),
    StoragePath(#[from] StoragePathError),
    ReadStorageItem(#[from] ReadStorageItemError),
}

pub async fn download_user_file(
    storage_root_dir: &PathBuf,
    path: &str,
    user: &User,
) -> Result<UserFileDownloadResult, UserFileDownloadError> {
    let user_storage = UserStorage {
        user_id: user.id,
        storage_root: storage_root_dir.clone(),
    };

    ensure_user_storage_exists(&user_storage).await?;

    let path = StoragePath::new(user_storage.clone(), PathBuf::from(path))?;
    let storage_item = read_storage_item(&path).await?;

    match storage_item.kind {
        StorageItemKind::File => {
            let mime = mime_guess::from_path(&path.scoped_path).first_or_octet_stream();
            let file = read_file_stream(&path).await?;

            Ok(UserFileDownloadResult {
                file: AsyncStorageItemRead::File(file),
                mime,
                path,
            })
        }
        StorageItemKind::Directory => {
            let (write_stream, read_stream) = tokio::io::duplex(4096);

            tokio::spawn(async move {
                match directory_zipping::write_zipped_storage_item_to_stream(
                    write_stream,
                    &storage_item,
                )
                .await
                {
                    Ok(()) => {}
                    Err(directory_zip_error) => {
                        tracing::event!(
                            Level::ERROR,
                            message = "Directory zipping failed",
                            error_trace = directory_zip_error.to_string(),
                        )
                    }
                };
            });

            let mut dir_path = path.scoped_path.clone();
            dir_path.set_extension("zip");

            let fake_zip_path = StoragePath::new(user_storage.clone(), PathBuf::from(dir_path))?;

            let mime = mime_guess::from_path(&fake_zip_path.scoped_path).first_or_octet_stream();

            Ok(UserFileDownloadResult {
                file: AsyncStorageItemRead::Directory(read_stream),
                mime,
                path: fake_zip_path,
            })
        }
    }
}
