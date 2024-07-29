use std::{sync::Arc, time::Duration};

use crate::AppState;
use axum::{
    extract::DefaultBodyLimit, http::StatusCode, response::IntoResponse, Extension, Router,
};
use tower::limit::RateLimitLayer;
use tower_http::trace::TraceLayer;

mod content;
mod static_assets;

pub(crate) async fn create_routes(_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(static_assets::create_routes().await)
        .merge(content::create_routes().await)
        .fallback(handler_404)
        .layer(Extension(RateLimitLayer::new(1000, Duration::from_secs(1))))
        .layer(DefaultBodyLimit::max(10240))
        .layer(TraceLayer::new_for_http())
}

pub async fn handler_404() -> impl IntoResponse {
    StatusCode::NOT_FOUND.into_response()
}

#[allow(dead_code)]
pub mod routes {
    use axum_route_helper::route;

    route!(Static => "/static/{}", (*file_path: String));

    route!(Home => "/");
}
