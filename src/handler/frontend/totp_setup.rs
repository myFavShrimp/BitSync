use std::sync::Arc;

use axum::{
    extract::State,
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
    Extension, Router,
};
use axum_extra::{extract::Form, routing::RouterExt};
use bitsync_core::use_case::user_settings::{
    retrieve_totp_setup_data::retrieve_totp_setup_data, setup_totp::setup_totp,
};
use bitsync_frontend::{
    pages::totp_setup::{TotpRecoveryCodesPrompt, TotpSetupPage},
    Render,
};
use serde::Deserialize;

use crate::{
    auth::{require_login_and_user_setup_middleware, AuthData},
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
    _: bitsync_routes::GetTotpSetupPage,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    match retrieve_totp_setup_data(&auth_data.user).await {
        Ok(totp_setup_data) => Html(TotpSetupPage::from(totp_setup_data).render().into_string()),
        Err(_) => todo!(),
    }
}

#[derive(Deserialize, Clone, Debug)]
struct TotpSetupFormData {
    totp: String,
}

async fn user_settings_totp_setup_submit_handler(
    _: bitsync_routes::PostTotpSetup,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    Form(totp_setup_data): Form<TotpSetupFormData>,
) -> impl IntoResponse {
    match setup_totp(&state.database, &auth_data.user, &totp_setup_data.totp).await {
        Ok(totp_setup_data) => Html(
            TotpRecoveryCodesPrompt::from(totp_setup_data)
                .render()
                .into_string(),
        ),
        Err(e) => todo!("{:?}", e),
    }
}
