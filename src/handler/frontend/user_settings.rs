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
    invite_token::{
        create_invite_token::create_invite_token, delete_invite_token::delete_invite_token,
        list_invite_tokens::list_invite_tokens,
    },
    user::{
        delete_user::delete_user, get_user::get_user, list_users::list_users,
        make_admin::make_admin, reset_user_totp::reset_user_totp, revoke_admin::revoke_admin,
        suspend_user::suspend_user, unsuspend_user::unsuspend_user,
    },
    user_settings::{
        list_sessions::list_sessions,
        terminate_all_other_sessions::terminate_all_other_sessions,
        terminate_session::{TerminateSessionOutcome, terminate_session},
        update_user_password::{UpdateUserPasswordError, update_user_password},
    },
    user_share::list_shared_paths::list_shared_paths,
};
use bitsync_frontend::{
    Component, DIALOG_WRAPPER_SELECTOR, Render,
    confirmation_dialog::ConfirmationDialog,
    pages::user_settings::{
        SettingsDialog, SettingsTab, SettingsTabArea,
        invites::InviteList,
        password::{PasswordDisplayError, PasswordTabContent},
        sessions::{SessionList, SessionsDisplayError},
        totp::{TotpDisplayError, TotpTabContent},
        users::UserList,
    },
};
use bitsync_hyperstim::{HyperStimCommand, HyperStimPatchMode};
use serde::Deserialize;

use crate::{
    AppState,
    auth::{
        AuthData, require_admin_login_and_totp_setup_middleware,
        require_login_and_totp_setup_middleware,
    },
    error_report::emit_error,
    handler::{RedirectHyperStim, internal_server_error_toast_response},
};

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(
            Router::new()
                .typed_get(user_settings_dialog_handler)
                .typed_get(user_settings_password_tab_handler)
                .typed_post(user_settings_password_change_handler)
                .typed_get(user_settings_sessions_tab_handler)
                .typed_post(user_settings_terminate_session_handler)
                .typed_post(user_settings_terminate_all_other_sessions_handler)
                .typed_get(user_settings_shares_tab_handler)
                .typed_get(user_settings_totp_tab_handler)
                .typed_post(user_settings_totp_initiate_handler)
                .typed_post(user_settings_totp_setup_handler)
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_login_and_totp_setup_middleware::<RedirectHyperStim>,
                ))
                .with_state(state.clone()),
        )
        .merge(
            Router::new()
                .typed_get(user_settings_users_tab_handler)
                .typed_get(confirm_make_admin_handler)
                .typed_post(user_settings_make_admin_handler)
                .typed_get(confirm_revoke_admin_handler)
                .typed_post(user_settings_remove_admin_handler)
                .typed_get(confirm_reset_user_totp_handler)
                .typed_post(user_settings_reset_user_totp_handler)
                .typed_get(confirm_suspend_user_handler)
                .typed_post(user_settings_suspend_user_handler)
                .typed_get(confirm_unsuspend_user_handler)
                .typed_post(user_settings_unsuspend_user_handler)
                .typed_get(confirm_delete_user_handler)
                .typed_post(user_settings_delete_user_handler)
                .typed_get(user_settings_invites_tab_handler)
                .typed_post(user_settings_invite_token_create_handler)
                .typed_post(user_settings_invite_token_delete_handler)
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_admin_login_and_totp_setup_middleware::<RedirectHyperStim>,
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
                is_admin: auth_data.user.is_admin,
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
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    let tab_area = SettingsTabArea {
        active_tab: SettingsTab::Password,
        is_admin: auth_data.user.is_admin,
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
                is_admin: auth_data.user.is_admin,
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

async fn user_settings_shares_tab_handler(
    _: bitsync_routes::GetUserSettingsSharesTab,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    match list_shared_paths(
        &state.database,
        &state.config.fs_storage_root_dir,
        &auth_data.user,
    )
    .await
    {
        Ok(shared_paths) => {
            let tab_area = SettingsTabArea {
                active_tab: SettingsTab::Shares { shared_paths },
                is_admin: auth_data.user.is_admin,
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
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    let tab_area = SettingsTabArea {
        active_tab: SettingsTab::Totp(TotpTabContent::Prompt),
        is_admin: auth_data.user.is_admin,
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

async fn user_settings_invites_tab_handler(
    _: bitsync_routes::GetUserSettingsInvitesTab,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    let invite_tokens = match list_invite_tokens(&state.database).await {
        Ok(invite_tokens) => invite_tokens,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let tab_area = SettingsTabArea {
        active_tab: SettingsTab::Invites { invite_tokens },
        is_admin: auth_data.user.is_admin,
    };

    Json(HyperStimCommand::HsPatchHtml {
        html: tab_area.render(),
        patch_target: tab_area.id_target(),
        patch_mode: HyperStimPatchMode::Outer,
    })
    .into_response()
}

async fn user_settings_invite_token_create_handler(
    _: bitsync_routes::PostUserSettingsInviteTokenCreate,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let invite_tokens = match create_invite_token(&state.database).await {
        Ok(invite_tokens) => invite_tokens,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let invite_list = InviteList {
        invite_tokens,
        error: None,
    };

    Json(HyperStimCommand::HsPatchHtml {
        html: invite_list.render(),
        patch_target: invite_list.id_target(),
        patch_mode: HyperStimPatchMode::Outer,
    })
    .into_response()
}

async fn user_settings_invite_token_delete_handler(
    path: bitsync_routes::PostUserSettingsInviteTokenDelete,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let invite_tokens = match delete_invite_token(&state.database, &path.invite_token_id).await {
        Ok(invite_tokens) => invite_tokens,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let invite_list = InviteList {
        invite_tokens,
        error: None,
    };

    Json(HyperStimCommand::HsPatchHtml {
        html: invite_list.render(),
        patch_target: invite_list.id_target(),
        patch_mode: HyperStimPatchMode::Outer,
    })
    .into_response()
}

async fn user_settings_users_tab_handler(
    _: bitsync_routes::GetUserSettingsUsersTab,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    let users = match list_users(&state.database, &auth_data.user.id).await {
        Ok(users) => users,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let tab_area = SettingsTabArea {
        active_tab: SettingsTab::Users { users },
        is_admin: auth_data.user.is_admin,
    };

    Json(HyperStimCommand::HsPatchHtml {
        html: tab_area.render(),
        patch_target: tab_area.id_target(),
        patch_mode: HyperStimPatchMode::Outer,
    })
    .into_response()
}

async fn user_settings_make_admin_handler(
    path: bitsync_routes::PostUserSettingsMakeAdmin,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    let users = match make_admin(&state.database, &path.user_id, &auth_data.user.id).await {
        Ok(users) => users,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let user_list = UserList { users };

    Json(HyperStimCommand::HsPatchHtml {
        html: user_list.render(),
        patch_target: user_list.id_target(),
        patch_mode: HyperStimPatchMode::Outer,
    })
    .into_response()
}

async fn user_settings_remove_admin_handler(
    path: bitsync_routes::PostUserSettingsRemoveAdmin,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    let users = match revoke_admin(&state.database, &path.user_id, &auth_data.user.id).await {
        Ok(users) => users,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let user_list = UserList { users };

    Json(HyperStimCommand::HsPatchHtml {
        html: user_list.render(),
        patch_target: user_list.id_target(),
        patch_mode: HyperStimPatchMode::Outer,
    })
    .into_response()
}

async fn user_settings_reset_user_totp_handler(
    path: bitsync_routes::PostUserSettingsResetUserTotp,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    let users = match reset_user_totp(&state.database, &path.user_id, &auth_data.user.id).await {
        Ok(users) => users,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let user_list = UserList { users };

    Json(HyperStimCommand::HsPatchHtml {
        html: user_list.render(),
        patch_target: user_list.id_target(),
        patch_mode: HyperStimPatchMode::Outer,
    })
    .into_response()
}

async fn user_settings_suspend_user_handler(
    path: bitsync_routes::PostUserSettingsSuspendUser,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    let users = match suspend_user(&state.database, &path.user_id, &auth_data.user.id).await {
        Ok(users) => users,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let user_list = UserList { users };

    Json(HyperStimCommand::HsPatchHtml {
        html: user_list.render(),
        patch_target: user_list.id_target(),
        patch_mode: HyperStimPatchMode::Outer,
    })
    .into_response()
}

async fn user_settings_unsuspend_user_handler(
    path: bitsync_routes::PostUserSettingsUnsuspendUser,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    let users = match unsuspend_user(&state.database, &path.user_id, &auth_data.user.id).await {
        Ok(users) => users,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let user_list = UserList { users };

    Json(HyperStimCommand::HsPatchHtml {
        html: user_list.render(),
        patch_target: user_list.id_target(),
        patch_mode: HyperStimPatchMode::Outer,
    })
    .into_response()
}

async fn user_settings_delete_user_handler(
    path: bitsync_routes::PostUserSettingsDeleteUser,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    let users = match delete_user(
        &state.database,
        &state.config.fs_storage_root_dir,
        &path.user_id,
        &auth_data.user.id,
    )
    .await
    {
        Ok(users) => users,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let user_list = UserList { users };

    Json(HyperStimCommand::HsPatchHtml {
        html: user_list.render(),
        patch_target: user_list.id_target(),
        patch_mode: HyperStimPatchMode::Outer,
    })
    .into_response()
}

async fn confirm_make_admin_handler(
    path: bitsync_routes::GetMakeAdminDialog,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user = match get_user(&state.database, &path.user_id).await {
        Ok(user) => user,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let action_url = bitsync_routes::PostUserSettingsMakeAdmin {
        user_id: path.user_id,
    }
    .to_string();

    let dialog = ConfirmationDialog {
        title: format!("Make {} Admin", user.username),
        message: format!(
            "This will grant admin privileges to {}. They will be able to manage users, invites, and other BitSync settings.",
            user.username
        ),
        confirm_label: "Make Admin".to_owned(),
        action_url,
        is_danger: false,
    };

    Json(HyperStimCommand::HsPatchHtml {
        html: dialog.render(),
        patch_target: DIALOG_WRAPPER_SELECTOR.to_owned(),
        patch_mode: HyperStimPatchMode::Append,
    })
    .into_response()
}

async fn confirm_revoke_admin_handler(
    path: bitsync_routes::GetRevokeAdminDialog,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user = match get_user(&state.database, &path.user_id).await {
        Ok(user) => user,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let action_url = bitsync_routes::PostUserSettingsRemoveAdmin {
        user_id: path.user_id,
    }
    .to_string();

    let dialog = ConfirmationDialog {
        title: format!("Revoke Admin for {}", user.username),
        message: format!(
            "This will remove admin privileges from {}. They will no longer be able to manage users, invites, and other BitSync settings.",
            user.username
        ),
        confirm_label: "Revoke Admin".to_owned(),
        action_url,
        is_danger: true,
    };

    Json(HyperStimCommand::HsPatchHtml {
        html: dialog.render(),
        patch_target: DIALOG_WRAPPER_SELECTOR.to_owned(),
        patch_mode: HyperStimPatchMode::Append,
    })
    .into_response()
}

async fn confirm_reset_user_totp_handler(
    path: bitsync_routes::GetResetUserTotpDialog,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user = match get_user(&state.database, &path.user_id).await {
        Ok(user) => user,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let action_url = bitsync_routes::PostUserSettingsResetUserTotp {
        user_id: path.user_id,
    }
    .to_string();

    let dialog = ConfirmationDialog {
        title: format!("Reset TOTP for {}", user.username),
        message: format!(
            "This will reset two-factor authentication for {} and sign them out. They will need to log in again and set it up.",
            user.username
        ),
        confirm_label: "Reset TOTP".to_owned(),
        action_url,
        is_danger: true,
    };

    Json(HyperStimCommand::HsPatchHtml {
        html: dialog.render(),
        patch_target: DIALOG_WRAPPER_SELECTOR.to_owned(),
        patch_mode: HyperStimPatchMode::Append,
    })
    .into_response()
}

async fn confirm_suspend_user_handler(
    path: bitsync_routes::GetSuspendUserDialog,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user = match get_user(&state.database, &path.user_id).await {
        Ok(user) => user,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let action_url = bitsync_routes::PostUserSettingsSuspendUser {
        user_id: path.user_id,
    }
    .to_string();

    let dialog = ConfirmationDialog {
        title: format!("Suspend {}", user.username),
        message: format!(
            "This will prevent {} from using BitSync. Their data will be preserved.",
            user.username
        ),
        confirm_label: "Suspend".to_owned(),
        action_url,
        is_danger: true,
    };

    Json(HyperStimCommand::HsPatchHtml {
        html: dialog.render(),
        patch_target: DIALOG_WRAPPER_SELECTOR.to_owned(),
        patch_mode: HyperStimPatchMode::Append,
    })
    .into_response()
}

async fn confirm_unsuspend_user_handler(
    path: bitsync_routes::GetUnsuspendUserDialog,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user = match get_user(&state.database, &path.user_id).await {
        Ok(user) => user,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let action_url = bitsync_routes::PostUserSettingsUnsuspendUser {
        user_id: path.user_id,
    }
    .to_string();

    let dialog = ConfirmationDialog {
        title: format!("Unsuspend {}", user.username),
        message: format!("This will allow {} to use BitSync again.", user.username),
        confirm_label: "Unsuspend".to_owned(),
        action_url,
        is_danger: false,
    };

    Json(HyperStimCommand::HsPatchHtml {
        html: dialog.render(),
        patch_target: DIALOG_WRAPPER_SELECTOR.to_owned(),
        patch_mode: HyperStimPatchMode::Append,
    })
    .into_response()
}

async fn confirm_delete_user_handler(
    path: bitsync_routes::GetDeleteUserDialog,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user = match get_user(&state.database, &path.user_id).await {
        Ok(user) => user,
        Err(error) => {
            emit_error(error);

            return internal_server_error_toast_response();
        }
    };

    let action_url = bitsync_routes::PostUserSettingsDeleteUser {
        user_id: path.user_id,
    }
    .to_string();

    let dialog = ConfirmationDialog {
        title: format!("Delete {}", user.username),
        message: format!(
            "This will permanently delete {} and all their data. This action cannot be undone.",
            user.username
        ),
        confirm_label: "Delete".to_owned(),
        action_url,
        is_danger: true,
    };

    Json(HyperStimCommand::HsPatchHtml {
        html: dialog.render(),
        patch_target: DIALOG_WRAPPER_SELECTOR.to_owned(),
        patch_mode: HyperStimPatchMode::Append,
    })
    .into_response()
}
