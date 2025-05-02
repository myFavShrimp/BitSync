use std::sync::Arc;

use axum::{
    extract::State,
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
    Extension, Router,
};
use axum_extra::{
    extract::{cookie::SameSite, CookieJar, Form},
    routing::RouterExt,
};
use axum_htmx::HxRequest;
use bitsync_core::use_case::auth::{
    registration::perform_registration, retrieve_totp_setup_data::retrieve_totp_setup_data,
    setup_totp::setup_totp,
};
use bitsync_frontend::{
    pages::{
        register::{RegisterForm, RegisterPage, TotpRecoveryCodesPrompt},
        totp_setup::TotpSetupPage,
    },
    Render,
};
use serde::Deserialize;

use crate::{
    auth::{require_login_and_no_totp_setup_middleware, require_logout_middleware, AuthData},
    handler::redirect_response,
};

use crate::AppState;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    let totp_setup_router = Router::new()
        .typed_get(register_totp_setup_page_handler)
        .typed_post(login_totp_setup_submit_handler)
        .route_layer(from_fn_with_state(
            state.clone(),
            require_login_and_no_totp_setup_middleware,
        ))
        .with_state(state.clone());

    Router::new()
        .typed_get(register_page_handler)
        .typed_post(register_action_handler)
        .route_layer(from_fn_with_state(state.clone(), require_logout_middleware))
        .merge(totp_setup_router)
        .with_state(state)
}

async fn register_page_handler(_: bitsync_routes::GetRegisterPage) -> impl IntoResponse {
    Html(RegisterPage::default().render().into_string())
}

#[derive(Deserialize, Clone, Debug)]
struct RegisterActionFormData {
    username: String,
    password: String,
}

async fn register_action_handler(
    _: bitsync_routes::PostRegisterAction,
    HxRequest(is_hx_request): HxRequest,
    State(state): State<Arc<AppState>>,
    cookie_jar: CookieJar,
    Form(registration_data): Form<RegisterActionFormData>,
) -> impl IntoResponse {
    match perform_registration(
        &state.database,
        &state.config.fs_storage_root_dir,
        &registration_data.username,
        &registration_data.password,
        state.config.auth.jwt_expiration_seconds,
        &state.config.auth.jwt_secret,
    )
    .await
    {
        Ok(result) => {
            let cookie_jar = cookie_jar.add({
                let mut auth_cookie = axum_extra::extract::cookie::Cookie::new(
                    crate::auth::AUTH_COOKIE_NAME,
                    result.jwt,
                );
                auth_cookie.set_same_site(SameSite::Strict);

                #[cfg(not(debug_assertions))]
                auth_cookie.set_secure(true);

                auth_cookie
            });

            (
                cookie_jar,
                redirect_response(is_hx_request, &bitsync_routes::GetLoginPage.to_string()),
            )
                .into_response()
        }
        Err(error) => RegisterPage::UserRegistration(RegisterForm {
            username: Some(registration_data.username),
            error_message: Some(error.to_string()),
        })
        .render()
        .into_string()
        .into_response(),
    }
}

async fn register_totp_setup_page_handler(
    _: bitsync_routes::GetRegisterTotpSetupPage,
    Extension(auth_data): Extension<AuthData>,
    HxRequest(is_hx_request): HxRequest,
) -> impl IntoResponse {
    match retrieve_totp_setup_data(&auth_data.user).await {
        Ok(totp_setup_data) => Html(TotpSetupPage::from(totp_setup_data).render().into_string()).into_response(),
        Err(error) => {
            match error {
                bitsync_core::use_case::auth::retrieve_totp_setup_data::RetrieveTotpSetupDataError::TotpAlreadySetUp(..) => redirect_response(is_hx_request, &bitsync_routes::GetFilesHomePage.to_string()),
                _ => todo!(),
            }
        },
    }
}

#[derive(Deserialize, Clone, Debug)]
struct TotpSetupFormData {
    totp: String,
}

async fn login_totp_setup_submit_handler(
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
            bitsync_core::use_case::auth::setup_totp::RetrieveTotpSetupDataError::TotpAlreadySetUp(..) => redirect_response(is_hx_request, &bitsync_routes::GetFilesHomePage.to_string()),
            _ => todo!(),
        },
    }
}
