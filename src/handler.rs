use std::sync::Arc;

use crate::AppState;
use axum::{
    Json, Router,
    extract::DefaultBodyLimit,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bitsync_hyperstim::HyperStimCommand;
use headers::Header;
use tower_http::{limit::RequestBodyLimitLayer, trace::TraceLayer};

mod frontend;
mod static_assets;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(static_assets::create_routes().await)
        .merge(frontend::create_routes(state.clone()).await)
        .fallback(handler_404)
        .layer(DefaultBodyLimit::max(10240000))
        .layer(RequestBodyLimitLayer::new(10240000))
        .layer(TraceLayer::new_for_http())
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, StatusCode::NOT_FOUND.to_string()).into_response()
}

pub fn http_redirect_response(redirect_route: &str) -> Response {
    (
        StatusCode::SEE_OTHER,
        [(headers::Location::name().as_str(), redirect_route)],
    )
        .into_response()
}

pub fn hyperstim_redirect_response(redirect_route: &str) -> Response {
    (
        StatusCode::OK,
        Json(HyperStimCommand::HsExecute {
            code: format!("window.location.href = '{}'", redirect_route),
        }),
    )
        .into_response()
}

#[deprecated]
pub fn redirect_response(is_hx_request: bool, redirect_route: &str) -> Response {
    match is_hx_request {
        true => hyperstim_redirect_response(redirect_route),
        false => http_redirect_response(redirect_route),
    }
}
