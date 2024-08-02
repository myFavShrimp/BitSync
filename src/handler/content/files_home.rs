use std::sync::Arc;

use axum::{
    middleware::from_extractor_with_state,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::{
    auth::{AuthStatus, RequireLogin},
    AppState,
};

use super::routes;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            &routes::FilesHome::handler_route(),
            get(files_home_page_handler),
        )
        .route_layer(from_extractor_with_state::<RequireLogin, Arc<AppState>>(
            state.clone(),
        ))
        .with_state(state)
}

#[derive(askama::Template)]
#[template(path = "files_home.html")]
struct FilesHome;

async fn files_home_page_handler(auth_status: AuthStatus) -> impl IntoResponse {
    tracing::debug!("{:#?}", auth_status);
    Html(FilesHome.to_string())
}
