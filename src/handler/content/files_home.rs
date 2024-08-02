use std::sync::Arc;

use axum::{
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::{
    auth::{require_login_middleware, AuthStatus},
    AppState,
};

use super::routes;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            &routes::FilesHome::handler_route(),
            get(files_home_page_handler),
        )
        .route_layer(from_fn_with_state(state.clone(), require_login_middleware))
        .with_state(state)
}

#[derive(askama::Template)]
#[template(path = "files_home.html")]
struct FilesHome;

async fn files_home_page_handler(auth_status: AuthStatus) -> impl IntoResponse {
    tracing::debug!("{:#?}", auth_status);
    Html(FilesHome.to_string())
}
