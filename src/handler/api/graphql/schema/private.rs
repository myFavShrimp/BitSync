use std::sync::Arc;

use async_graphql::dataloader::{DataLoader, HashMapCache};

use crate::{database::user::User, AppState};

use dataloader::UserLoader;

// pub mod mutation;
// mod object;
pub mod dataloader;
pub mod query;

pub struct Context {
    pub app_state: Arc<AppState>,
    pub current_user: User,
    pub dataloader: DataLoader<UserLoader, HashMapCache>,
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
