use std::sync::Arc;

use crate::AppState;

// pub mod mutation;
// mod object;
pub mod query;

pub struct Context {
    pub app_state: Arc<AppState>,
}

pub type Root = async_graphql::Schema<
    query::Query,
    async_graphql::EmptyMutation,
    async_graphql::EmptySubscription,
>;

pub fn create_root() -> Root {
    async_graphql::Schema::build(
        query::Query,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
    .finish()
}
