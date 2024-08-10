use std::sync::Arc;

use axum::{
    extract::{Query, State},
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
    Router,
};
use axum_extra::routing::RouterExt;
use serde::Deserialize;

use crate::{
    auth::{require_login_middleware, AuthData},
    handler::routes::GetFilesHomePageQueryParameters,
    presentation::StorageItemPresentation,
    use_case, AppState,
};

use super::routes;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_get(files_home_page_handler)
        .route_layer(from_fn_with_state(state.clone(), require_login_middleware))
        .with_state(state)
}

#[derive(askama::Template)]
#[template(path = "files_home.html")]
struct FilesHome {
    dir_content: Vec<StorageItemPresentation>,
}

async fn files_home_page_handler(
    _: routes::GetFilesHomePage,
    State(app_state): State<Arc<AppState>>,
    auth_data: AuthData,
    query_parameters: Query<GetFilesHomePageQueryParameters>,
) -> impl IntoResponse {
    match use_case::user_files::user_directory(&app_state, &auth_data, &query_parameters.path).await
    {
        Ok(dir_content) => {
            let displayable_dir_content = dir_content
                .into_iter()
                .map(StorageItemPresentation::from)
                .collect();

            Html(
                FilesHome {
                    dir_content: displayable_dir_content,
                }
                .to_string(),
            )
        }
        Err(_) => todo!(),
    }
}
