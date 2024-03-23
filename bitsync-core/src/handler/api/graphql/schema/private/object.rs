use time::OffsetDateTime;

use crate::storage::{DirItem, FileItem, StorageItemPath};

mod use_case;

#[async_graphql::Object]
impl DirItem {
    async fn path(&self) -> &StorageItemPath {
        &self.path
    }

    async fn updated_at(&self) -> &OffsetDateTime {
        &self.updated_at
    }

    async fn files<'context>(
        &self,
        ctx: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Vec<FileItem>> {
        Ok(use_case::dir_item::list_files(
            ctx,
            &self.path.scoped_path.to_string_lossy(),
            self.content.clone(),
        )
        .await?)
    }

    async fn directories<'context>(
        &self,
        ctx: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Vec<DirItem>> {
        Ok(use_case::dir_item::list_directories(
            ctx,
            &self.path.scoped_path.to_string_lossy(),
            self.content.clone(),
        )
        .await?)
    }
}
