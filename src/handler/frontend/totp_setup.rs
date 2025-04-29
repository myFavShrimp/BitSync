use std::sync::Arc;

use axum::{
    extract::State,
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
    Extension, Router,
};
use axum_extra::{extract::Form, routing::RouterExt};
use axum_htmx::HxRequest;
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
    handler::redirect_response,
    AppState,
};

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_get(totp_setup_page_handler)
        .typed_post(user_settings_totp_setup_submit_handler)
        .route_layer(from_fn_with_state(
            state.clone(),
            require_login_and_user_setup_middleware,
        ))
        .with_state(state)
}

async fn totp_setup_page_handler(
    _: bitsync_routes::GetRegisterTotpSetupPage,
    Extension(auth_data): Extension<AuthData>,
    HxRequest(is_hx_request): HxRequest,
) -> impl IntoResponse {
    match retrieve_totp_setup_data(&auth_data.user).await {
        Ok(totp_setup_data) => Html(TotpSetupPage::from(totp_setup_data).render().into_string()).into_response(),
        Err(error) => {
            match error {
                bitsync_core::use_case::user_settings::retrieve_totp_setup_data::RetrieveTotpSetupDataError::TotpAlreadySetUp(..) => redirect_response(is_hx_request, &bitsync_routes::GetFilesHomePage.to_string()),
                _ => todo!(),
            }
        },
    }
}

#[derive(Deserialize, Clone, Debug)]
struct TotpSetupFormData {
    totp: String,
}

async fn user_settings_totp_setup_submit_handler(
    _: bitsync_routes::PostRegisterTotpSetupAction,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    HxRequest(is_hx_request): HxRequest,
    Form(totp_setup_data): Form<TotpSetupFormData>,
) -> impl IntoResponse {
    match setup_totp(&state.database, &auth_data.user, &totp_setup_data.totp).await {
        Ok(totp_setup_data) => Html(
            TotpRecoveryCodesPrompt::from(totp_setup_data)
                .render()
                .into_string(),
        ).into_response(),
        Err(error) => match error {
            bitsync_core::use_case::user_settings::setup_totp::RetrieveTotpSetupDataError::TotpAlreadySetUp(..) => redirect_response(is_hx_request, &bitsync_routes::GetFilesHomePage.to_string()),
            _ => todo!(),
        },
    }
}
