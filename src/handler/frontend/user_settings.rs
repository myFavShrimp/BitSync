use std::sync::Arc;

use axum::{
    Extension, Json, Router, extract::State, http::StatusCode, middleware::from_fn_with_state,
    response::IntoResponse,
};
use axum_extra::{extract::Form, routing::RouterExt};
use bitsync_core::use_case::user_settings::{
    list_sessions::list_sessions,
    terminate_all_other_sessions::terminate_all_other_sessions,
    terminate_session::{TerminateSessionError, terminate_session},
    update_user_password::{UpdateUserPasswordError, update_user_password},
};
use bitsync_frontend::{
    Component, DIALOG_WRAPPER_SELECTOR, Render,
    pages::user_settings::{
        SettingsDialog, SettingsTab, SettingsTabArea,
        password::{PasswordDisplayError, PasswordTabContent},
        sessions::{SessionList, SessionsDisplayError},
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
        Ok(sessions) => (StatusCode::OK, sessions, None),
        Err(TerminateSessionError::CannotTerminateCurrentSession(..)) => {
            let sessions = match list_sessions(&state.database, &auth_data.user.id).await {
                Ok(sessions) => sessions,
                Err(error) => {
                    emit_error(error);
                    return internal_server_error_toast_response();
                }
            };

            (
                StatusCode::BAD_REQUEST,
                sessions,
                Some(SessionsDisplayError::CannotTerminateCurrentSession),
            )
        }
        Err(error) => {
            emit_error(error);

            let sessions = match list_sessions(&state.database, &auth_data.user.id).await {
                Ok(sessions) => sessions,
                Err(error) => {
                    emit_error(error);
                    return internal_server_error_toast_response();
                }
            };

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                sessions,
                Some(SessionsDisplayError::InternalServerError),
            )
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
    let result =
        terminate_all_other_sessions(&state.database, &auth_data.user.id, &auth_data.session.id)
            .await;

    let (status_code, sessions, display_error) = match result {
        Ok(sessions) => (StatusCode::OK, sessions, None),
        Err(error) => {
            emit_error(error);

            let sessions = match list_sessions(&state.database, &auth_data.user.id).await {
                Ok(sessions) => sessions,
                Err(error) => {
                    emit_error(error);
                    return internal_server_error_toast_response();
                }
            };

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                sessions,
                Some(SessionsDisplayError::InternalServerError),
            )
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
