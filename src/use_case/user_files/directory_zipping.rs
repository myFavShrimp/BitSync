use tokio::io::DuplexStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::storage::{
    DirContentsError, FileContentsError, StorageBackend, StorageItem, StorageItemKind,
};

#[derive(Debug, thiserror::Error)]
#[error("Failed to copy a stream")]
pub struct StreamCopyError(#[from] std::io::Error);

#[derive(Debug, thiserror::Error)]
#[error("An error occurred while writing a directory zip")]
pub enum DirectoryZipError {
    DirContents(#[from] DirContentsError),
    Zip(#[from] async_zip::error::ZipError),
    FileContents(#[from] FileContentsError),
    StreamCopy(#[from] StreamCopyError),
}

pub async fn write_zipped_storage_item_to_stream(
    stream: DuplexStream,
    storage_item: &StorageItem,
) -> Result<(), DirectoryZipError> {
    let mut zip_file_writer = async_zip::tokio::write::ZipFileWriter::with_tokio(stream);

    write_storage_item_to_zip(&mut zip_file_writer, storage_item, storage_item).await?;

    zip_file_writer.close().await?;

    Ok(())
}

#[async_recursion::async_recursion]
async fn write_storage_item_to_zip(
    zip_file_writer: &mut async_zip::tokio::write::ZipFileWriter<DuplexStream>,
    storage_item: &StorageItem,
    root_storage_item: &StorageItem,
) -> Result<(), DirectoryZipError> {
    match storage_item.kind {
        crate::storage::StorageItemKind::File => {
            let zipped_item_path = if let StorageItemKind::Directory = root_storage_item.kind {
                storage_item
                    .path
                    .scoped_path
                    .strip_prefix(&root_storage_item.path.scoped_path)
                    .map(|path| path.to_path_buf())
                    .unwrap_or(storage_item.path.scoped_path.clone())
            } else {
                storage_item.path.scoped_path.clone()
            };

            let zip_entry_builder = async_zip::ZipEntryBuilder::new(
                async_zip::ZipString::from(zipped_item_path.to_string_lossy().to_string()),
                async_zip::Compression::Stored,
            );

            let mut zip_entry_writer = zip_file_writer
                .write_entry_stream(zip_entry_builder)
                .await?;

            let file_stream = StorageBackend::file_stream(&storage_item.path).await?;

            futures::io::copy(&mut file_stream.compat(), &mut zip_entry_writer)
                .await
                .map_err(StreamCopyError)?;

            zip_entry_writer.close().await?;
        }
        crate::storage::StorageItemKind::Directory => {
            let directory_contents = StorageBackend::dir_contents(&storage_item.path).await?;

            for directory_item in directory_contents {
                write_storage_item_to_zip(zip_file_writer, &directory_item, root_storage_item)
                    .await?;
            }
        }
    };

    Ok(())
}
