use crate::database::user::User;

use super::Context;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn me<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<User> {
        let context = context.data::<Context>()?;

        Ok(context.current_user.clone())
    }

    async fn users<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Vec<User>> {
        let context = context.data::<Context>()?;

        let users = User::find_all(&context.app_state.postgres_pool).await?;

        context
            .dataloader
            .feed_many(users.iter().map(|user| (user.id, user.clone())))
            .await;

        Ok(users)
    }
}
