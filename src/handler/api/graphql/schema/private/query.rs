use super::Context;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn hello<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<String> {
        let _context = context.data::<Context>()?;

        Ok(String::from("World"))
    }
}
