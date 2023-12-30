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

pub async fn list_my_storage_items<'context>(
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
        crate::storage::StorageItem::DirItem(mut dir_item) => {
            let is_directories_query = ctx.look_ahead().field("directories").exists();
            let is_files_query = ctx.look_ahead().field("files").exists();

            if is_directories_query || is_files_query {
                let dir_content = storage.list_storage_items(path).await?;

                let mut directories = Vec::new();
                let mut files = Vec::new();

                for item in dir_content {
                    match item {
                        crate::storage::StorageItem::DirItem(dir_item) => {
                            directories.push(dir_item)
                        }
                        crate::storage::StorageItem::FileItem(file_item) => files.push(file_item),
                    }
                }

                dir_item.directories = directories;
                dir_item.files = files;
            }

            Ok(dir_item)
        }
    }
}
