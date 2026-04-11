use std::sync::Arc;

use axum::{
    Extension, Json, Router, extract::State, http::StatusCode, middleware::from_fn_with_state,
    response::IntoResponse,
};
use axum_extra::{extract::Form, routing::RouterExt};
use bitsync_core::use_case::{
    auth::{
        initiate_totp_setup::initiate_totp_setup,
        reset_totp::{ResetTotpError, reset_totp},
    },
    user_settings::{
        list_sessions::list_sessions,
        terminate_all_other_sessions::terminate_all_other_sessions,
        terminate_session::{TerminateSessionOutcome, terminate_session},
        update_user_password::{UpdateUserPasswordError, update_user_password},
    },
};
use bitsync_frontend::{
    Component, DIALOG_WRAPPER_SELECTOR, Render,
    pages::user_settings::{
        SettingsDialog, SettingsTab, SettingsTabArea,
        password::{PasswordDisplayError, PasswordTabContent},
        sessions::{SessionList, SessionsDisplayError},
        totp::{TotpDisplayError, TotpTabContent},
    },
};
use bitsync_hyperstim::{HyperStimCommand, HyperStimPatchMode};
use serde::Deserialize;

use crate::{
    AppState,
    auth::{AuthData, require_login_and_totp_setup_middleware},
    error_report::emit_error,
    handler::{RedirectHyperStim, internal_server_error_toast_response},
};

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new().merge(
        Router::new()
            .typed_get(user_settings_dialog_handler)
            .typed_get(user_settings_password_tab_handler)
            .typed_post(user_settings_password_change_handler)
            .typed_get(user_settings_sessions_tab_handler)
            .typed_post(user_settings_terminate_session_handler)
            .typed_post(user_settings_terminate_all_other_sessions_handler)
            .typed_get(user_settings_totp_tab_handler)
            .typed_post(user_settings_totp_initiate_handler)
            .typed_post(user_settings_totp_setup_handler)
            .route_layer(from_fn_with_state(
                state.clone(),
                require_login_and_totp_setup_middleware::<RedirectHyperStim>,
            ))
            .with_state(state.clone()),
    )
}

async fn user_settings_dialog_handler(
    _: bitsync_routes::GetUserSettingsDialog,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    match list_sessions(&state.database, &auth_data.user.id).await {
        Ok(sessions) => {
            let dialog = SettingsDialog {
                sessions,
                current_session_id: auth_data.session.id,
            };

            Json(HyperStimCommand::HsPatchHtml {
                html: dialog.render(),
                patch_target: DIALOG_WRAPPER_SELECTOR.to_owned(),
                patch_mode: HyperStimPatchMode::Append,
            })
            .into_response()
        }
        Err(error) => {
            emit_error(error);

            internal_server_error_toast_response()
        }
    }
}

async fn user_settings_password_tab_handler(
    _: bitsync_routes::GetUserSettingsPasswordTab,
    Extension(_auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    let tab_area = SettingsTabArea {
        active_tab: SettingsTab::Password,
    };

    Json(HyperStimCommand::HsPatchHtml {
        html: tab_area.render(),
        patch_target: tab_area.id_target(),
        patch_mode: HyperStimPatchMode::Outer,
    })
}

async fn user_settings_sessions_tab_handler(
    _: bitsync_routes::GetUserSettingsSessionsTab,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    match list_sessions(&state.database, &auth_data.user.id).await {
        Ok(sessions) => {
            let tab_area = SettingsTabArea {
                active_tab: SettingsTab::Sessions {
                    sessions,
                    current_session_id: auth_data.session.id,
                },
            };

            Json(HyperStimCommand::HsPatchHtml {
                html: tab_area.render(),
                patch_target: tab_area.id_target(),
                patch_mode: HyperStimPatchMode::Outer,
            })
            .into_response()
        }
        Err(error) => {
            emit_error(error);

            internal_server_error_toast_response()
        }
    }
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
    let result = update_user_password(
        &state.database,
        &auth_data.user,
        &change_password_form_data.current_password,
        &change_password_form_data.new_password,
        &change_password_form_data.new_password_repeated,
        &auth_data.session.id,
    )
    .await;

    let (status_code, display_error) = match result {
        Ok(()) => (StatusCode::OK, None),
        Err(UpdateUserPasswordError::PasswordHashVerification(..)) => (
            StatusCode::BAD_REQUEST,
            Some(PasswordDisplayError::InvalidCurrentPassword),
        ),
        Err(UpdateUserPasswordError::NewPasswordsMismatch(..)) => (
            StatusCode::BAD_REQUEST,
            Some(PasswordDisplayError::NewPasswordsMismatch),
        ),
        Err(UpdateUserPasswordError::EmptyNewPassword(..)) => (
            StatusCode::BAD_REQUEST,
            Some(PasswordDisplayError::EmptyNewPassword),
        ),
        Err(error) => {
            emit_error(error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(PasswordDisplayError::InternalServerError),
            )
        }
    };

    let form = PasswordTabContent {
        error: display_error,
    };

    (
        status_code,
        Json(HyperStimCommand::HsPatchHtml {
            html: form.render(),
            patch_target: form.id_target(),
            patch_mode: HyperStimPatchMode::Outer,
        }),
    )
        .into_response()
}

async fn user_settings_terminate_session_handler(
    path: bitsync_routes::PostTerminateSession,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    let result = terminate_session(
        &state.database,
        &auth_data.user.id,
        &path.session_id,
        &auth_data.session.id,
    )
    .await;

    let (status_code, sessions, display_error) = match result {
        Ok(TerminateSessionOutcome::Terminated(sessions)) => (StatusCode::OK, sessions, None),
        Ok(TerminateSessionOutcome::CannotTerminateCurrentSession(sessions)) => (
            StatusCode::BAD_REQUEST,
            sessions,
            Some(SessionsDisplayError::CannotTerminateCurrentSession),
        ),
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let sessions_list = SessionList {
        sessions,
        current_session_id: auth_data.session.id,
        error: display_error,
    };

    (
        status_code,
        Json(HyperStimCommand::HsPatchHtml {
            html: sessions_list.render(),
            patch_target: sessions_list.id_target(),
            patch_mode: HyperStimPatchMode::Outer,
        }),
    )
        .into_response()
}

async fn user_settings_terminate_all_other_sessions_handler(
    _: bitsync_routes::PostTerminateAllOtherSessions,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    let sessions = match terminate_all_other_sessions(
        &state.database,
        &auth_data.user.id,
        &auth_data.session.id,
    )
    .await
    {
        Ok(sessions) => sessions,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let sessions_list = SessionList {
        sessions,
        current_session_id: auth_data.session.id,
        error: None,
    };

    Json(HyperStimCommand::HsPatchHtml {
        html: sessions_list.render(),
        patch_target: sessions_list.id_target(),
        patch_mode: HyperStimPatchMode::Outer,
    })
    .into_response()
}

async fn user_settings_totp_tab_handler(
    _: bitsync_routes::GetUserSettingsTotpTab,
) -> impl IntoResponse {
    let tab_area = SettingsTabArea {
        active_tab: SettingsTab::Totp(TotpTabContent::Prompt),
    };

    Json(HyperStimCommand::HsPatchHtml {
        html: tab_area.render(),
        patch_target: tab_area.id_target(),
        patch_mode: HyperStimPatchMode::Outer,
    })
}

async fn user_settings_totp_initiate_handler(
    _: bitsync_routes::PostUserSettingsTotpInitiateReset,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    let totp_setup_data = match initiate_totp_setup(&state.database, &auth_data.user.id).await {
        Ok(data) => data,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let content = TotpTabContent::Setup {
        totp_secret_image_base64_img_src: totp_setup_data.secret_base64_qr_code,
        totp_secret: totp_setup_data.secret_base32,
        error: None,
    };

    Json(HyperStimCommand::HsPatchHtml {
        html: content.render(),
        patch_target: content.id_target(),
        patch_mode: HyperStimPatchMode::Outer,
    })
    .into_response()
}

#[derive(Deserialize)]
struct TotpSetupFormData {
    totp: String,
}

async fn user_settings_totp_setup_handler(
    _: bitsync_routes::PostUserSettingsTotpReset,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    Form(totp_setup_data): Form<TotpSetupFormData>,
) -> impl IntoResponse {
    let result = reset_totp(&state.database, &auth_data.user, &totp_setup_data.totp).await;

    match result {
        Ok(reset_result) => {
            let content = TotpTabContent::RecoveryCodes {
                recovery_codes: reset_result.recovery_codes,
            };

            Json(HyperStimCommand::HsPatchHtml {
                html: content.render(),
                patch_target: content.id_target(),
                patch_mode: HyperStimPatchMode::Outer,
            })
            .into_response()
        }
        Err(ResetTotpError::InvalidTotpCode(error)) => {
            let content = TotpTabContent::Setup {
                totp_secret_image_base64_img_src: error.setup_data.secret_base64_qr_code,
                totp_secret: error.setup_data.secret_base32,
                error: Some(TotpDisplayError::InvalidCode),
            };

            (
                StatusCode::BAD_REQUEST,
                Json(HyperStimCommand::HsPatchHtml {
                    html: content.render(),
                    patch_target: content.id_target(),
                    patch_mode: HyperStimPatchMode::Outer,
                }),
            )
                .into_response()
        }
        Err(ResetTotpError::NoTotpResetInProgress(..)) => {
            let content = TotpTabContent::Prompt;

            Json(HyperStimCommand::HsPatchHtml {
                html: content.render(),
                patch_target: content.id_target(),
                patch_mode: HyperStimPatchMode::Outer,
            })
            .into_response()
        }
        Err(error) => {
            emit_error(error);

            internal_server_error_toast_response()
        }
    }
}
