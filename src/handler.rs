use std::{sync::Arc, time::Duration};

use crate::AppState;
use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Router,
};
use headers::Header;
use tower::limit::RateLimitLayer;
use tower_http::trace::TraceLayer;

mod frontend;
mod static_assets;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(static_assets::create_routes().await)
        .merge(frontend::create_routes(state).await)
        .fallback(handler_404)
        .layer(Extension(RateLimitLayer::new(1000, Duration::from_secs(1))))
        .layer(DefaultBodyLimit::max(10240))
        .layer(TraceLayer::new_for_http())
}

pub async fn handler_404() -> impl IntoResponse {
    StatusCode::NOT_FOUND.into_response()
}

pub fn http_redirect_response(redirect_route: &str) -> Response {
    (
        StatusCode::SEE_OTHER,
        [(headers::Location::name().as_str(), redirect_route)],
    )
        .into_response()
}

pub fn htmx_redirect_response(redirect_route: &str) -> Response {
    (StatusCode::OK, [("HX-Redirect", redirect_route)]).into_response()
}

pub fn redirect_response(is_hx_request: bool, redirect_route: &str) -> Response {
    match is_hx_request {
        true => htmx_redirect_response(redirect_route),
        false => http_redirect_response(redirect_route),
    }
}
