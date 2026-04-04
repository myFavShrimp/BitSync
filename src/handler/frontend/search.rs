use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Query, State},
    middleware::from_fn_with_state,
    response::IntoResponse,
};
use axum_extra::routing::RouterExt;
use bitsync_core::use_case::user_files::search_user_files::{
    SearchUserFilesResult, search_user_files,
};
use bitsync_frontend::{
    Render,
    pages::search::{SearchResultItem, SearchResults},
    toast::{TOAST_CONTAINER_SELECTOR, Toast},
};
use bitsync_hyperstim::{HyperStimCommand, HyperStimPatchMode};

use crate::{
    AppState,
    auth::{AuthData, require_login_and_totp_setup_middleware},
    handler::RedirectHyperStim,
};

static SEARCH_RESULTS_SELECTOR: &str = "#search-results";

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_get(search_handler)
        .route_layer(from_fn_with_state(
            state.clone(),
            require_login_and_totp_setup_middleware::<RedirectHyperStim>,
        ))
        .with_state(state)
}

async fn search_handler(
    _: bitsync_routes::GetSearch,
    State(app_state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    query_parameters: Query<bitsync_routes::GetSearchQueryParameters>,
) -> impl IntoResponse {
    match search_user_files(
        &app_state.config.fs_storage_root_dir,
        &query_parameters.query,
        &auth_data.user,
        query_parameters.path.as_deref(),
    )
    .await
    {
        Ok(SearchUserFilesResult::NoSearch) => Json(HyperStimCommand::HsPatchHtml {
            html: String::new(),
            patch_target: SEARCH_RESULTS_SELECTOR.to_owned(),
            patch_mode: HyperStimPatchMode::Inner,
        })
        .into_response(),
        Ok(SearchUserFilesResult::Results {
            current_dir_results,
            global_results,
        }) => {
            let search_results = SearchResults {
                current_dir_items: current_dir_results
                    .into_iter()
                    .map(|search_result| {
                        SearchResultItem::new(search_result.storage_item, search_result.parent_path)
                    })
                    .collect(),
                global_items: global_results
                    .into_iter()
                    .map(|search_result| {
                        SearchResultItem::new(search_result.storage_item, search_result.parent_path)
                    })
                    .collect(),
            };

            Json(HyperStimCommand::HsPatchHtml {
                html: search_results.render(),
                patch_target: SEARCH_RESULTS_SELECTOR.to_owned(),
                patch_mode: HyperStimPatchMode::Inner,
            })
            .into_response()
        }
        Err(_) => Json(vec![
            // TODO: display error inline in results
            HyperStimCommand::HsPatchHtml {
                html: Toast::error("Search failed").render(),
                patch_target: TOAST_CONTAINER_SELECTOR.to_owned(),
                patch_mode: HyperStimPatchMode::Append,
            },
        ])
        .into_response(),
    }
}
