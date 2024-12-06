use std::sync::Arc;

use axum::{
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
    Router,
};
use axum_extra::routing::RouterExt;

use crate::{
    auth::{require_login_middleware, AuthData},
    handler::routes::GetUserSettingsPage,
    presentation::templates::user_settings_page::UserSettingsPage,
    AppState,
};

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_get(account_page_handler)
        .route_layer(from_fn_with_state(state.clone(), require_login_middleware))
        .with_state(state)
}

async fn account_page_handler(_: GetUserSettingsPage, auth_data: AuthData) -> impl IntoResponse {
    Html(UserSettingsPage::from(auth_data.user).to_string())
}
