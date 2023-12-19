use crate::database::user::User;

use super::Context;

mod use_case;

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn register<'context>(
        &self,
        context: &async_graphql::Context<'context>,
        username: String,
        password: String,
    ) -> async_graphql::Result<User> {
        let context = context.data::<Context>()?;

        Ok(use_case::register::perform_registration(context, username, password).await?)
    }
}
