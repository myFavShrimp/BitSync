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
}
