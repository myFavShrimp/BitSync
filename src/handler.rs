use std::{sync::Arc, time::Duration};

use crate::AppState;
use axum::{
    extract::DefaultBodyLimit, http::StatusCode, response::IntoResponse, Extension, Router,
};
use tower::limit::RateLimitLayer;
use tower_http::trace::TraceLayer;

mod content;
mod static_assets;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(static_assets::create_routes().await)
        .merge(content::create_routes(state).await)
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
    use axum_extra::routing::TypedPath;
    use serde::{Deserialize, Serialize};

    fn build_default_files_query_parameter_path() -> String {
        "/".to_owned()
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/static/*file_path")]
    pub struct Static {
        pub file_path: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/")]
    pub struct GetFilesHomePage;
    #[derive(Deserialize, Serialize, Debug)]
    pub struct GetFilesHomePageQueryParameters {
        #[serde(default = "build_default_files_query_parameter_path")]
        pub path: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/login")]
    pub struct GetLoginPage;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/login")]
    pub struct PostLoginAction;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/logout")]
    pub struct GetLogoutAction;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/register")]
    pub struct GetRegisterPage;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/register")]
    pub struct PostRegisterAction;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct GetUserFileDownloadQueryParameters {
        #[serde(default = "build_default_files_query_parameter_path")]
        pub path: String,
    }
    #[derive(TypedPath, Deserialize)]
    #[typed_path("/user_file/download")]
    pub struct GetUserFileDownload;
}
