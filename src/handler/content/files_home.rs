use std::sync::Arc;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::AppState;

use super::routes;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            &routes::FilesHome::handler_route(),
            get(files_home_page_handler),
        )
        .with_state(state)
}

#[derive(askama::Template)]
#[template(path = "files_home.html")]
struct FilesHome;

async fn files_home_page_handler() -> impl IntoResponse {
    Html(FilesHome.to_string())
}
