use crate::database::user::User;

use super::Context;

mod use_case;

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn update_password<'context>(
        &self,
        context: &async_graphql::Context<'context>,
        new_password: String,
    ) -> async_graphql::Result<User> {
        let context = context.data::<Context>()?;

        Ok(use_case::user_settings::update_password(context, &new_password).await?)
    }
}
