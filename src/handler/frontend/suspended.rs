use std::sync::Arc;

use axum::{
    Router,
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
};
use axum_extra::routing::RouterExt;
use bitsync_frontend::{Render, pages::suspended::SuspendedPage};

use crate::{AppState, auth::require_suspended_middleware, handler::RedirectHttp};

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_get(suspended_page_handler)
        .route_layer(from_fn_with_state(
            state.clone(),
            require_suspended_middleware::<RedirectHttp>,
        ))
        .with_state(state)
}

async fn suspended_page_handler(_: bitsync_routes::GetSuspendedPage) -> impl IntoResponse {
    Html(SuspendedPage.render())
}
