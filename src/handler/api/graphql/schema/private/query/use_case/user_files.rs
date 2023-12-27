use opendal::Metakey;

use crate::{
    directory::user_data_directory,
    dto::File,
    handler::api::graphql::PrivateContext,
    validate::{sanitize_directory_path, validate_file_path, PathValidationError},
};

#[derive(thiserror::Error, Debug)]
pub enum UserDirectoryReadError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error(transparent)]
    Opendal(#[from] opendal::Error),
    #[error(transparent)]
    PathValidation(#[from] PathValidationError),
}

pub async fn list_my_directory<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
) -> Result<Vec<File>, UserDirectoryReadError> {
    validate_file_path(path)?;
    let path = sanitize_directory_path(path);

    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserDirectoryReadError::Context)?;

    let user_directory = user_data_directory(
        context.app_state.config.fs_storage_root_dir.clone(),
        &context.current_user.id,
    );
    let mut fs_storage_dir = user_directory.clone();
    fs_storage_dir.push(&path.strip_prefix('/').unwrap_or(&path));

    let mut fs_builder = opendal::services::Fs::default();
    fs_builder.root(fs_storage_dir.to_str().expect("Path is valid UTF-8"));

    let op = (opendal::Operator::new(fs_builder))?.finish();

    Ok(op
        .list_with(&path)
        .recursive(true)
        .metakey(
            Metakey::Complete
                | Metakey::Mode
                // | Metakey::CacheControl
                // | Metakey::ContentDisposition
                | Metakey::ContentLength
                | Metakey::ContentMd5
                | Metakey::ContentRange
                | Metakey::ContentType
                // | Metakey::Etag
                | Metakey::LastModified
                | Metakey::Version,
        )
        .await?
        .into_iter()
        .map(|entry| (entry.path().to_string(), entry.metadata().clone()).into())
        .collect())
}
