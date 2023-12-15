use std::sync::Arc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::AppState;

use super::routes;

mod dataloader;

pub async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            &routes::ApiGraphQl::handler_route(),
            get(api_graphql_get_handler).post(api_graphql_post_handler),
        )
        .with_state(state)
}

pub async fn api_graphql_get_handler() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        &routes::ApiGraphQl::route_path(),
    )))
}

pub async fn api_graphql_post_handler() -> &'static str {
    "Hello, World!"
}
