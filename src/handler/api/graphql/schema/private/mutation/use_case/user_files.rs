use async_graphql::Upload;

use crate::{
    dto::DirectoryEntry,
    handler::api::graphql::PrivateContext,
    storage::{user_data_directory, Storage, StorageError},
};

#[derive(thiserror::Error, Debug)]
pub enum UserFileUploadError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error("Error handling the file upload - {0}")]
    Storage(#[from] StorageError),
    #[error("Could not read the file data")]
    FileUploadRead(std::io::Error),
}

pub async fn upload_user_file<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
    mut files: Vec<Upload>,
) -> Result<Vec<DirectoryEntry>, UserFileUploadError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserFileUploadError::Context)?;

    let user_directory = user_data_directory(
        context.app_state.config.fs_storage_root_dir.clone(),
        &context.current_user.id,
    );

    let files: Result<Vec<async_graphql::UploadValue>, std::io::Error> =
        files.iter_mut().map(|file| file.value(ctx)).collect();

    let storage = Storage {
        storage_root: user_directory,
    };

    let mut result = Vec::new();
    for file in files.map_err(UserFileUploadError::FileUploadRead)? {
        let file_name = &file.filename;
        let file_content = file.content;

        result.push(storage.add_file(path, file_name, file_content).await?);
    }

    Ok(result)
}
