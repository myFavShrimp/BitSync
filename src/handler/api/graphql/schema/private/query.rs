use crate::{database::user::User, storage::DirItem};

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

    async fn list_my_storage_items<'context>(
        &self,
        ctx: &async_graphql::Context<'context>,
        path: String,
    ) -> async_graphql::Result<DirItem> {
        Ok(use_case::user_files::list_my_storage_items(ctx, &path).await?)
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
