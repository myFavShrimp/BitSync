use std::sync::Arc;

use axum::{
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
    Extension, Router,
};
use axum_extra::routing::RouterExt;

use crate::{
    auth::{require_login_and_user_setup_middleware, AuthData},
    handler::routes::GetTotpSetupPage,
    presentation::templates::user_settings_page::UserSettingsPage,
    AppState,
};

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_get(user_settings_totp_setup_page_handler)
        .route_layer(from_fn_with_state(
            state.clone(),
            require_login_and_user_setup_middleware,
        ))
        .with_state(state)
}

async fn user_settings_totp_setup_page_handler(
    _: GetTotpSetupPage,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    Html(UserSettingsPage::from(auth_data.user).to_string())
}
