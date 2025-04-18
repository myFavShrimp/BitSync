use std::sync::Arc;

use axum::{
    extract::{Query, State},
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
    Extension, Router,
};
use axum_extra::routing::RouterExt;
use bitsync_core::use_case::user_files::read_user_directory_contents::read_user_directory_contents;
use bitsync_frontend::{
    pages::{error::ErrorPage, files::FilesHomePage},
    Render,
};

use crate::{
    auth::{require_login_and_user_setup_middleware, AuthData},
    AppState,
};

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_get(files_home_page_handler)
        .route_layer(from_fn_with_state(
            state.clone(),
            require_login_and_user_setup_middleware,
        ))
        .with_state(state)
}

async fn files_home_page_handler(
    _: bitsync_routes::GetFilesHomePage,
    State(app_state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    query_parameters: Query<bitsync_routes::GetFilesHomePageQueryParameters>,
) -> impl IntoResponse {
    match read_user_directory_contents(
        &app_state.config.fs_storage_root_dir,
        &query_parameters.path,
        &auth_data.user,
    )
    .await
    {
        Ok(result) => Html(FilesHomePage::from(result).render().into_string()),
        Err(error) => Html(ErrorPage::from(error).render().into_string()),
    }
}
