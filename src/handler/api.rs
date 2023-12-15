use std::sync::Arc;

use axum::Router;

use crate::AppState;

use super::routes;

mod graphql;

pub async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new().merge(graphql::create_routes(state.clone()).await)
}
