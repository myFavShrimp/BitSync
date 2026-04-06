use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Query, State},
    http::StatusCode,
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
};
use axum_extra::{
    extract::{CookieJar, Form},
    routing::RouterExt,
};
use bitsync_core::use_case::auth::{
    redeem_invite_token::{RedeemInviteTokenError, redeem_invite_token},
    registration::{RegistrationError, perform_registration},
    retrieve_totp_setup_data::{RetrieveTotpSetupDataError, retrieve_totp_setup_data},
    setup_totp::{TotpSetupError, setup_totp},
};
use bitsync_frontend::{
    Component, Render,
    pages::register::{
        InviteTokenDisplayError, InviteTokenForm, RegisterForm, RegisterPage,
        RegistrationDisplayError, TotpRecoveryCodesPrompt, TotpSetupDisplayError, TotpSetupForm,
    },
};
use bitsync_hyperstim::{HyperStimCommand, HyperStimPatchMode};
use bitsync_routes::TypedPath;
use serde::Deserialize;

use crate::{
    auth::{
        AuthData, jwt_cookie, require_login_and_no_totp_setup_middleware, require_logout_middleware,
    },
    error_report::emit_error,
    handler::{RedirectHttp, RedirectHyperStim, hyperstim_redirect_response},
};

use crate::AppState;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(
            Router::new()
                .typed_get(register_totp_setup_page_handler)
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_login_and_no_totp_setup_middleware::<RedirectHttp>,
                )),
        )
        .merge(
            Router::new()
                .typed_post(register_totp_setup_submit_handler)
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_login_and_no_totp_setup_middleware::<RedirectHyperStim>,
                ))
                .with_state(state.clone()),
        )
        .merge(
            Router::new()
                .typed_get(register_page_handler)
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_logout_middleware::<RedirectHttp>,
                )),
        )
        .merge(
            Router::new()
                .typed_post(redeem_invite_token_handler)
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_logout_middleware::<RedirectHyperStim>,
                ))
                .with_state(state.clone()),
        )
        .merge(
            Router::new()
                .typed_post(register_action_handler)
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_logout_middleware::<RedirectHyperStim>,
                ))
                .with_state(state.clone()),
        )
}

async fn register_page_handler(
    _: bitsync_routes::GetRegisterPage,
    Query(query_parameters): Query<bitsync_routes::GetRegisterPageQueryParameters>,
) -> impl IntoResponse {
    match query_parameters.token {
        Some(token) => Html(
            RegisterPage::UserRegistration(RegisterForm {
                token,
                username: None,
                error: None,
            })
            .render(),
        ),
        None => Html(RegisterPage::default().render()),
    }
}

#[derive(Deserialize, Clone, Debug)]
struct RedeemInviteTokenFormData {
    token: String,
}

async fn redeem_invite_token_handler(
    _: bitsync_routes::PostRedeemInviteToken,
    State(state): State<Arc<AppState>>,
    Form(form_data): Form<RedeemInviteTokenFormData>,
) -> impl IntoResponse {
    match redeem_invite_token(&state.database, &form_data.token).await {
        Ok(()) => {
            let redirect_url = bitsync_routes::GetRegisterPage
                .with_query_params(bitsync_routes::GetRegisterPageQueryParameters {
                    token: Some(form_data.token.clone()),
                })
                .to_string();
            hyperstim_redirect_response(&redirect_url).into_response()
        }
        Err(RedeemInviteTokenError::InvalidInviteTokenError(..)) => {
            let invite_token_form = InviteTokenForm {
                error: Some(InviteTokenDisplayError::InvalidToken),
            };

            (
                StatusCode::BAD_REQUEST,
                Json(HyperStimCommand::HsPatchHtml {
                    html: invite_token_form.render(),
                    patch_target: invite_token_form.id_target(),
                    patch_mode: HyperStimPatchMode::Outer,
                }),
            )
                .into_response()
        }
        Err(error) => {
            emit_error(error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
struct RegisterActionFormData {
    username: String,
    password: String,
    platform: String,
}

async fn register_action_handler(
    _: bitsync_routes::PostRegisterAction,
    State(state): State<Arc<AppState>>,
    Query(query_parameters): Query<bitsync_routes::PostRegisterActionQueryParameters>,
    cookie_jar: CookieJar,
    Form(registration_data): Form<RegisterActionFormData>,
) -> impl IntoResponse {
    let token_uuid = match uuid::Uuid::parse_str(&query_parameters.token) {
        Ok(uuid) => uuid,
        Err(_) => {
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    match perform_registration(
        &state.database,
        &state.config.fs_storage_root_dir,
        &registration_data.username,
        &registration_data.password,
        &token_uuid,
        &registration_data.platform,
        state.config.auth.jwt_expiration_seconds,
        &state.config.auth.jwt_secret,
    )
    .await
    {
        Ok(result) => {
            let cookie_jar = cookie_jar.add(jwt_cookie(&result.jwt));

            (
                cookie_jar,
                hyperstim_redirect_response(&bitsync_routes::GetRegisterTotpSetupPage.to_string()),
            )
                .into_response()
        }
        Err(error) => {
            let display_error = match error {
                RegistrationError::UserExists(..) => RegistrationDisplayError::UsernameTaken,
                RegistrationError::InvalidInviteTokenError(..) => {
                    RegistrationDisplayError::InvalidInviteToken
                }
                error => {
                    emit_error(error);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            };

            let register_form = RegisterForm {
                token: query_parameters.token,
                username: Some(registration_data.username),
                error: Some(display_error),
            };

            (
                StatusCode::CONFLICT,
                Json(HyperStimCommand::HsPatchHtml {
                    html: register_form.render(),
                    patch_target: register_form.id_target(),
                    patch_mode: HyperStimPatchMode::Outer,
                }),
            )
                .into_response()
        }
    }
}

async fn register_totp_setup_page_handler(
    _: bitsync_routes::GetRegisterTotpSetupPage,
    Extension(auth_data): Extension<AuthData>,
) -> impl IntoResponse {
    match retrieve_totp_setup_data(&auth_data.user).await {
        Ok(totp_setup_data) => Html(
            RegisterPage::TotpSetup(TotpSetupForm {
                totp_secret_image_base64_img_src: totp_setup_data.secret_base64_qr_code,
                totp_secret: totp_setup_data.secret_base32,
                error: None,
            })
            .render(),
        )
        .into_response(),
        Err(error) => match error {
            RetrieveTotpSetupDataError::TotpAlreadySetUp(..) => {
                hyperstim_redirect_response(&bitsync_routes::GetFilesHomePage.to_string())
            }
            error => {
                emit_error(error);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        },
    }
}

#[derive(Deserialize, Clone, Debug)]
struct TotpSetupFormData {
    totp: String,
}

async fn register_totp_setup_submit_handler(
    _: bitsync_routes::PostRegisterTotpSetupAction,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    cookie_jar: CookieJar,
    Form(totp_setup_data): Form<TotpSetupFormData>,
) -> impl IntoResponse {
    match setup_totp(
        &state.database,
        &auth_data.user,
        &auth_data.session.id,
        &totp_setup_data.totp,
        state.config.auth.jwt_expiration_seconds,
        &state.config.auth.jwt_secret,
    )
    .await
    {
        Ok(result) => {
            let cookie_jar = cookie_jar.add(jwt_cookie(&result.jwt));

            (cookie_jar, {
                let totp_prompt = TotpRecoveryCodesPrompt::from(result);

                Json(HyperStimCommand::HsPatchHtml {
                    html: totp_prompt.render(),
                    patch_target: totp_prompt.id_target(),
                    patch_mode: HyperStimPatchMode::Outer,
                })
            })
                .into_response()
        }
        Err(error) => match error {
            TotpSetupError::TotpAlreadySetUp(..) => {
                hyperstim_redirect_response(&bitsync_routes::GetFilesHomePage.to_string())
            }
            TotpSetupError::TotpInvalid(..) => {
                match retrieve_totp_setup_data(&auth_data.user).await {
                    Ok(totp_setup_data) => {
                        let totp_form = TotpSetupForm {
                            totp_secret_image_base64_img_src: totp_setup_data.secret_base64_qr_code,
                            totp_secret: totp_setup_data.secret_base32,
                            error: Some(TotpSetupDisplayError::InvalidCode),
                        };

                        Json(HyperStimCommand::HsPatchHtml {
                            html: totp_form.render(),
                            patch_target: totp_form.id_target(),
                            patch_mode: HyperStimPatchMode::Outer,
                        })
                        .into_response()
                    }
                    Err(error) => {
                        emit_error(error);
                        StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    }
                }
            }
            error => {
                emit_error(error);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        },
    }
}
