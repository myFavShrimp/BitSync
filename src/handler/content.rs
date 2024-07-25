use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use super::routes;

pub(crate) async fn create_routes() -> Router {
    Router::new().route(&routes::Home::handler_route(), get(index_handler))
}

#[derive(askama::Template)]
#[template(path = "index.html")]
struct Index;

async fn index_handler() -> impl IntoResponse {
    Html(Index.to_string())
}
