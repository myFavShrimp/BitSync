use std::sync::Arc;

use crate::AppState;
use axum::{
    Json, Router,
    extract::DefaultBodyLimit,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bitsync_frontend::{
    Render,
    toast::{TOAST_CONTAINER_SELECTOR, Toast},
};
use bitsync_hyperstim::{HyperStimCommand, HyperStimPatchMode};
use headers::Header;
use tower_http::{limit::RequestBodyLimitLayer, trace::TraceLayer};

mod frontend;
mod static_assets;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(static_assets::create_routes().await)
        .merge(frontend::create_routes(state.clone()).await)
        .fallback(handler_404)
        .layer(DefaultBodyLimit::disable())
        .layer(DefaultBodyLimit::max(10_240_000))
        .layer(RequestBodyLimitLayer::new(10_240_000))
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

pub enum RedirectionKind {
    Http,
    HyperStim,
}

pub trait Redirection {
    fn redirection_kind() -> RedirectionKind;
}

struct RedirectHttp;

impl Redirection for RedirectHttp {
    fn redirection_kind() -> RedirectionKind {
        RedirectionKind::Http
    }
}

struct RedirectHyperStim;

impl Redirection for RedirectHyperStim {
    fn redirection_kind() -> RedirectionKind {
        RedirectionKind::HyperStim
    }
}

pub fn redirect_response<KIND: Redirection>(redirect_route: &str) -> Response {
    match KIND::redirection_kind() {
        RedirectionKind::HyperStim => hyperstim_redirect_response(redirect_route),
        RedirectionKind::Http => http_redirect_response(redirect_route),
    }
}

pub fn internal_server_error_toast_response() -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(HyperStimCommand::HsPatchHtml {
            html: Toast::error("An internal server error occurred").render(),
            patch_target: TOAST_CONTAINER_SELECTOR.to_owned(),
            patch_mode: HyperStimPatchMode::Append,
        }),
    )
        .into_response()
}

pub fn user_error_toast_response(message: impl Into<String>) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(HyperStimCommand::HsPatchHtml {
            html: Toast::error(message).render(),
            patch_target: TOAST_CONTAINER_SELECTOR.to_owned(),
            patch_mode: HyperStimPatchMode::Append,
        }),
    )
        .into_response()
}
