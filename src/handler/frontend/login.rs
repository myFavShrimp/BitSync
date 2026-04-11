use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::State,
    http::StatusCode,
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
};
use axum_extra::{
    TypedHeader,
    extract::{CookieJar, Form},
    routing::RouterExt,
};
use bitsync_core::use_case::auth::{
    login::{LoginError, perform_login},
    verify_totp::{VerifyTotpError, verify_totp},
};
use bitsync_frontend::{
    Component, Render,
    pages::login::{
        LoginDisplayError, LoginForm, LoginPage, TotpForm, TotpVerificationDisplayError,
    },
};
use bitsync_hyperstim::{HyperStimCommand, HyperStimPatchMode};
use serde::Deserialize;

use crate::{
    auth::{
        AuthData, jwt_cookie, require_basic_login_and_totp_setup_middleware,
        require_logout_middleware,
    },
    error_report::emit_error,
    handler::{RedirectHttp, RedirectHyperStim, hyperstim_redirect_response},
};

use crate::AppState;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(
            Router::new()
                .typed_get(login_totp_auth_page_handler)
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_basic_login_and_totp_setup_middleware::<RedirectHttp>,
                )),
        )
        .merge(
            Router::new()
                .typed_post(login_totp_auth_submit_handler)
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_basic_login_and_totp_setup_middleware::<RedirectHyperStim>,
                ))
                .with_state(state.clone()),
        )
        .merge(
            Router::new()
                .typed_get(login_page_handler)
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_logout_middleware::<RedirectHttp>,
                )),
        )
        .merge(
            Router::new()
                .typed_post(login_action_handler)
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_logout_middleware::<RedirectHyperStim>,
                ))
                .with_state(state.clone()),
        )
}

async fn login_page_handler(_: bitsync_routes::GetLoginPage) -> impl IntoResponse {
    Html(LoginPage::default().render())
}

#[derive(Deserialize, Clone, Debug)]
struct LoginActionFormData {
    username: String,
    password: String,
}

async fn login_action_handler(
    _: bitsync_routes::PostLoginAction,
    State(state): State<Arc<AppState>>,
    TypedHeader(user_agent): axum_extra::TypedHeader<headers::UserAgent>,
    cookie_jar: CookieJar,
    Form(login_data): Form<LoginActionFormData>,
) -> impl IntoResponse {
    match perform_login(
        &state.database,
        &login_data.username,
        &login_data.password,
        user_agent.as_str(),
        &state.config.auth.jwt_secret,
    )
    .await
    {
        Ok(result) => {
            let cookie_jar = cookie_jar.add(jwt_cookie(&result.jwt));

            let redirect_url = match result.user.is_totp_set_up {
                true => bitsync_routes::GetLoginTotpAuthPage.to_string(),
                false => bitsync_routes::GetRegisterTotpSetupPage.to_string(),
            };

            (cookie_jar, hyperstim_redirect_response(&redirect_url)).into_response()
        }
        Err(error) => {
            let (status_code, display_error) = match error {
                LoginError::PasswordHashVerification(..) | LoginError::UserNotFound(..) => (
                    StatusCode::UNAUTHORIZED,
                    LoginDisplayError::InvalidCredentials,
                ),
                error => {
                    emit_error(error);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        LoginDisplayError::InternalServerError,
                    )
                }
            };

            let login_form = LoginForm {
                username: Some(login_data.username),
                error: Some(display_error),
            };

            (
                status_code,
                Json(HyperStimCommand::HsPatchHtml {
                    html: login_form.render(),
                    patch_target: login_form.id_target(),
                    patch_mode: HyperStimPatchMode::Outer,
                }),
            )
                .into_response()
        }
    }
}

async fn login_totp_auth_page_handler(
    _: bitsync_routes::GetLoginTotpAuthPage,
) -> impl IntoResponse {
    Html(LoginPage::Totp(TotpForm { error: None }).render())
}

#[derive(Deserialize, Clone, Debug)]
struct TotpAuthFormData {
    totp: String,
}

async fn login_totp_auth_submit_handler(
    _: bitsync_routes::PostLoginTotpAuthAction,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    cookie_jar: CookieJar,
    Form(totp_setup_data): Form<TotpAuthFormData>,
) -> impl IntoResponse {
    match verify_totp(
        &auth_data.user,
        &auth_data.session.id,
        &totp_setup_data.totp,
        &state.config.auth.jwt_secret,
    )
    .await
    {
        Ok(jwt) => {
            let cookie_jar = cookie_jar.add(jwt_cookie(&jwt));

            (
                cookie_jar,
                hyperstim_redirect_response(&bitsync_routes::GetFilesHomePage.to_string()),
            )
                .into_response()
        }
        Err(error) => {
            let (status_code, display_error) = match error {
                VerifyTotpError::TotpInvalid(..) => (
                    StatusCode::UNAUTHORIZED,
                    TotpVerificationDisplayError::InvalidCode,
                ),
                VerifyTotpError::TotpNotSetUp(..) => (
                    StatusCode::UNAUTHORIZED,
                    TotpVerificationDisplayError::NotSetUp,
                ),
                error => {
                    emit_error(error);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        TotpVerificationDisplayError::InternalServerError,
                    )
                }
            };

            let totp_form = TotpForm {
                error: Some(display_error),
            };

            (
                status_code,
                Json(HyperStimCommand::HsPatchHtml {
                    html: totp_form.render(),
                    patch_target: totp_form.id_target(),
                    patch_mode: HyperStimPatchMode::Outer,
                }),
            )
                .into_response()
        }
    }
}
