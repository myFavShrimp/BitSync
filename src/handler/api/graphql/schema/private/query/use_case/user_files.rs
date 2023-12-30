use crate::{
    dto::DirectoryEntry,
    handler::api::graphql::PrivateContext,
    storage::{user_data_directory, Storage, StorageError},
};

#[derive(thiserror::Error, Debug)]
pub enum UserDirectoryReadError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error(transparent)]
    Storage(#[from] StorageError),
}

pub async fn list_my_storage_items<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
) -> Result<Vec<DirectoryEntry>, UserDirectoryReadError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserDirectoryReadError::Context)?;

    let user_directory = user_data_directory(
        context.app_state.config.fs_storage_root_dir.clone(),
        &context.current_user.id,
    );

    Ok(Storage {
        storage_root: user_directory,
    }
    .list_storage_items(path)
    .await?)
}
