use std::sync::Arc;

use axum::{
    extract::{Query, State},
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

use crate::{
    auth::{require_login_middleware, AuthData},
    storage::StorageItem,
    use_case, AppState,
};

use super::routes;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            &routes::GetFilesHomePage::handler_route(),
            get(files_home_page_handler),
        )
        .route_layer(from_fn_with_state(state.clone(), require_login_middleware))
        .with_state(state)
}

fn build_default_files_home_query_parameter_path() -> String {
    "/".to_owned()
}

#[derive(Deserialize, Debug)]
struct FilesHomePageQueryParameters {
    #[serde(default = "build_default_files_home_query_parameter_path")]
    path: String,
}

#[derive(askama::Template)]
#[template(path = "files_home.html")]
struct FilesHome {
    dir_content: Vec<StorageItem>,
}

async fn files_home_page_handler(
    State(app_state): State<Arc<AppState>>,
    auth_data: AuthData,
    query_parameters: Query<FilesHomePageQueryParameters>,
) -> impl IntoResponse {
    match use_case::user_files::user_directory(&app_state, &auth_data, &query_parameters.path).await
    {
        Ok(dir_content) => Html(FilesHome { dir_content }.to_string()),
        Err(_) => todo!(),
    }
}
