use std::sync::Arc;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::AppState;

use super::routes;

mod login;
mod register;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(&routes::Home::handler_route(), get(index_handler))
        .merge(login::create_routes(state.clone()).await)
        .merge(register::create_routes(state).await)
}

#[derive(askama::Template)]
#[template(path = "index.html")]
struct Index;

async fn index_handler() -> impl IntoResponse {
    Html(Index.to_string())
}
