use std::sync::Arc;

use axum::{
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
    Extension, Router,
};
use axum_extra::{extract::Form, routing::RouterExt};
use bitsync_core::use_case::user_settings::retrieve_totp_setup_data::retrieve_totp_setup_data;
use serde::Deserialize;

use crate::{
    auth::{require_login_and_user_setup_middleware, AuthData},
    handler::routes::{GetTotpSetupPage, PostTotpSetup},
    presentation::templates::user_settings_page::totp_setup_page::{TotpSetupForm, TotpSetupPage},
    AppState,
};

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_get(user_settings_totp_setup_page_handler)
        .typed_post(user_settings_totp_setup_submit_handler)
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
    match retrieve_totp_setup_data(&auth_data.user).await {
        Ok(totp_setup_data) => Html(TotpSetupPage::from(totp_setup_data).to_string()),
        Err(_) => todo!(),
    }
}

#[derive(Deserialize, Clone, Debug)]
struct TotpSetupFormData {
    totp: String,
}

async fn user_settings_totp_setup_submit_handler(
    _: PostTotpSetup,
    Extension(auth_data): Extension<AuthData>,
    Form(login_data): Form<TotpSetupFormData>,
) -> impl IntoResponse {
    match retrieve_totp_setup_data(&auth_data.user).await {
        Ok(totp_setup_data) => Html(TotpSetupForm::from(totp_setup_data).to_string()),
        Err(_) => todo!(),
    }
}
