use crate::{
    database::user::User, handler::api::graphql::schema::FormattedStringError, storage::DirItem,
};

use self::use_case::user_files::UserStorageItemSearchResult;

use super::Context;

mod use_case;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn me<'context>(
        &self,
        ctx: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<User> {
        let context = ctx.data::<Context>()?;

        Ok(context.current_user.clone())
    }

    async fn user_directory<'context>(
        &self,
        ctx: &async_graphql::Context<'context>,
        path: String,
    ) -> async_graphql::Result<DirItem> {
        Ok(use_case::user_files::user_directory(ctx, &path).await?)
    }

    async fn user_search_storage_items<'context>(
        &self,
        ctx: &async_graphql::Context<'context>,
        search: String,
    ) -> async_graphql::Result<UserStorageItemSearchResult> {
        Ok(use_case::user_files::user_storage_item_search(ctx, &search)
            .await
            .to_formatted_string_error()?)
    }

    async fn users<'context>(
        &self,
        ctx: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Vec<User>> {
        let context = ctx.data::<Context>()?;

        let users = User::find_all(&context.app_state.postgres_pool).await?;

        context
            .dataloader
            .feed_many(users.iter().map(|user| (user.id, user.clone())))
            .await;

        Ok(users)
    }
}
