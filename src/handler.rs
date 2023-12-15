use std::{sync::Arc, time::Duration};

use axum::{
    extract::DefaultBodyLimit, http::StatusCode, response::IntoResponse, Extension, Router,
};
use tower::limit::RateLimitLayer;
use tower_http::trace::TraceLayer;

use crate::AppState;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .fallback(handler_404)
        .layer(Extension(RateLimitLayer::new(1000, Duration::from_secs(1))))
        .layer(DefaultBodyLimit::max(10240))
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone())
}

pub async fn handler_404() -> impl IntoResponse {
    StatusCode::NOT_FOUND.into_response()
}
