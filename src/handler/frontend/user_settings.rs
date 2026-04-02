use std::sync::Arc;

use axum::{
    Extension, Json, Router, extract::State, http::StatusCode, middleware::from_fn_with_state,
    response::IntoResponse,
};
use axum_extra::{extract::Form, routing::RouterExt};
use bitsync_core::use_case::user_settings::update_user_password::{
    UpdateUserPasswordError, update_user_password,
};
use bitsync_frontend::{DIALOG_WRAPPER_SELECTOR, Render, pages::user_settings::SettingsDialog};
use bitsync_hyperstim::{HyperStimCommand, HyperStimPatchMode};
use serde::Deserialize;

use crate::{
    AppState,
    auth::{AuthData, require_login_and_totp_setup_middleware},
    error_report::emit_error,
    handler::RedirectHyperStim,
};

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(
            Router::new()
                .typed_get(user_settings_page_handler)
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_login_and_totp_setup_middleware::<RedirectHyperStim>,
                )),
        )
        .merge(
            Router::new()
                .typed_post(user_settings_password_change_handler)
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_login_and_totp_setup_middleware::<RedirectHyperStim>,
                ))
                .with_state(state.clone()),
        )
}

async fn user_settings_page_handler(
    _: bitsync_routes::GetUserSettingsPage,
    Extension(_auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    Json(HyperStimCommand::HsPatchHtml {
        html: SettingsDialog.render(),
        patch_target: DIALOG_WRAPPER_SELECTOR.to_owned(),
        patch_mode: HyperStimPatchMode::Append,
    })
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
        Ok(()) => StatusCode::OK.into_response(),
        Err(UpdateUserPasswordError::PasswordHashVerification(..)) => {
            StatusCode::BAD_REQUEST.into_response()
        }
        Err(UpdateUserPasswordError::PasswordsMismatch(..)) => {
            StatusCode::BAD_REQUEST.into_response()
        }
        Err(error) => {
            emit_error(error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
