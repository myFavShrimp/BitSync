use super::Context;

mod use_case;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn login<'context>(
        &self,
        context: &async_graphql::Context<'context>,
        username: String,
        password: String,
    ) -> async_graphql::Result<String> {
        let context = context.data::<Context>()?;

        Ok(use_case::login::perform_login(context, username, password).await?)
    }
}
