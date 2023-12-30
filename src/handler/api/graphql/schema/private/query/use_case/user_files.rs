use crate::{
    handler::api::graphql::PrivateContext,
    storage::{user_data_directory, DirItem, Storage, StorageError},
};

#[derive(thiserror::Error, Debug)]
pub enum UserDirectoryReadError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error("Not a directory")]
    NotADirectory,
}

pub async fn user_directory<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
) -> Result<DirItem, UserDirectoryReadError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserDirectoryReadError::Context)?;

    let user_directory = user_data_directory(
        context.app_state.config.fs_storage_root_dir.clone(),
        &context.current_user.id,
    );

    let storage = Storage {
        storage_root: user_directory,
    };

    let storage_item = storage.storage_item(path).await?;

    match storage_item {
        crate::storage::StorageItem::FileItem(_) => Err(UserDirectoryReadError::NotADirectory),
        crate::storage::StorageItem::DirItem(dir_item) => Ok(dir_item),
    }
}
