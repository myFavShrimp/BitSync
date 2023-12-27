use crate::{database::user::User, dto::File};

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

    async fn list_my_directory<'context>(
        &self,
        ctx: &async_graphql::Context<'context>,
        path: String,
    ) -> async_graphql::Result<Vec<File>> {
        Ok(use_case::user_files::list_my_directory(ctx, &path).await?)
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
