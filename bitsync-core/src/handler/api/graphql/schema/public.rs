use std::sync::Arc;

use async_graphql::extensions::Tracing;

use crate::AppState;

pub mod mutation;
mod object;
pub mod query;

pub struct Context {
    pub app_state: Arc<AppState>,
}

pub type Root =
    async_graphql::Schema<query::Query, mutation::Mutation, async_graphql::EmptySubscription>;

pub fn create_root() -> Root {
    async_graphql::Schema::build(
        query::Query,
        mutation::Mutation,
        async_graphql::EmptySubscription,
    )
    .extension(Tracing)
    .finish()
}
