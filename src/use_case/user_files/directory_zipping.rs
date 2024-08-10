use tokio::io::DuplexStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::storage::{DirContentsError, FileContentsError, StorageBackend, StorageItem};

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
    directory_item: &StorageItem,
) -> Result<(), DirectoryZipError> {
    let mut zip_file_writer = async_zip::tokio::write::ZipFileWriter::with_tokio(stream);

    write_storage_item_to_zip(&mut zip_file_writer, directory_item).await?;

    zip_file_writer.close().await?;

    Ok(())
}

#[async_recursion::async_recursion]
async fn write_storage_item_to_zip(
    zip_file_writer: &mut async_zip::tokio::write::ZipFileWriter<DuplexStream>,
    storage_item: &StorageItem,
) -> Result<(), DirectoryZipError> {
    match storage_item.kind {
        crate::storage::StorageItemKind::File => {
            let zip_entry_builder = async_zip::ZipEntryBuilder::new(
                async_zip::ZipString::from(storage_item.path.path()),
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
            let contents = StorageBackend::dir_contents(&storage_item.path).await?;

            for storage_item in contents {
                write_storage_item_to_zip(zip_file_writer, &storage_item).await?;
            }
        }
    };

    Ok(())
}
