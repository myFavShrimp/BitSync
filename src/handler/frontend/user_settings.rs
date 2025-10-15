use std::sync::Arc;

use axum::{
    Extension, Router,
    extract::State,
    http::StatusCode,
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
};
use axum_extra::{extract::Form, routing::RouterExt};
use bitsync_core::use_case::user_settings::update_user_password::update_user_password;
use bitsync_frontend::{Render, pages::user_settings::UserSettingsPage};
use serde::Deserialize;

use crate::{
    AppState,
    auth::{AuthData, require_login_and_totp_setup_middleware},
};

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_get(user_settings_page_handler)
        .typed_post(user_settings_password_change_handler)
        .route_layer(from_fn_with_state(
            state.clone(),
            require_login_and_totp_setup_middleware,
        ))
        .with_state(state.clone())
}

async fn user_settings_page_handler(
    _: bitsync_routes::GetUserSettingsPage,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    Html(UserSettingsPage::from(auth_data.user).render())
}

#[derive(Deserialize)]
struct ChangePasswordFormData {
    pub current_password: String,
    pub new_password: String,
    pub new_password_repeated: String,
}

async fn user_settings_password_change_handler(
    _: bitsync_routes::PostUserSettingsChangePassword,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    Form(change_password_form_data): Form<ChangePasswordFormData>,
) -> impl IntoResponse {
    match update_user_password(
        &state.database,
        &auth_data.user,
        &change_password_form_data.current_password,
        &change_password_form_data.new_password,
        &change_password_form_data.new_password_repeated,
    )
    .await
    {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => todo!("{:#?}", e),
    }
}
