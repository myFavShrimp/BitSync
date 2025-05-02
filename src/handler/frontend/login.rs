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
use bitsync_core::use_case::auth::{login::perform_login, verify_totp::verify_totp};
use bitsync_frontend::{
    pages::login::{LoginPage, TotpForm},
    Render,
};
use serde::Deserialize;

use crate::{
    auth::{
        jwt_cookie, require_basic_login_and_totp_setup_middleware, require_logout_middleware,
        AuthData,
    },
    handler::redirect_response,
};

use crate::AppState;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    let totp_auth_router = Router::new()
        .typed_get(login_totp_auth_page_handler)
        .typed_post(login_totp_auth_submit_handler)
        .route_layer(from_fn_with_state(
            state.clone(),
            require_basic_login_and_totp_setup_middleware,
        ))
        .with_state(state.clone());

    Router::new()
        .typed_get(login_page_handler)
        .typed_post(login_action_handler)
        .route_layer(from_fn_with_state(state.clone(), require_logout_middleware))
        .merge(totp_auth_router)
        .with_state(state)
}

async fn login_page_handler(_: bitsync_routes::GetLoginPage) -> impl IntoResponse {
    Html(LoginPage::default().render().into_string())
}

#[derive(Deserialize, Clone, Debug)]
struct LoginActionFormData {
    username: String,
    password: String,
}

async fn login_action_handler(
    _: bitsync_routes::PostLoginAction,
    State(state): State<Arc<AppState>>,
    HxRequest(is_hx_request): HxRequest,
    cookie_jar: CookieJar,
    Form(login_data): Form<LoginActionFormData>,
) -> impl IntoResponse {
    match perform_login(
        &state.database,
        &login_data.username,
        &login_data.password,
        state.config.auth.jwt_expiration_seconds,
        &state.config.auth.jwt_secret,
    )
    .await
    {
        Ok(result) => {
            let cookie_jar = cookie_jar.add(jwt_cookie(result.jwt));

            let redirect_url = match result.user.is_totp_set_up {
                true => bitsync_routes::GetFilesHomePage.to_string(),
                false => bitsync_routes::GetRegisterTotpSetupPage.to_string(),
            };

            (cookie_jar, redirect_response(is_hx_request, &redirect_url))
        }
        Err(e) => todo!("login error handling - {:#?}", e),
    }
}

async fn login_totp_auth_page_handler(
    _: bitsync_routes::GetLoginTotpAuthPage,
) -> impl IntoResponse {
    Html(
        LoginPage::Totp(TotpForm {
            error_message: None,
        })
        .render()
        .into_string(),
    )
}

#[derive(Deserialize, Clone, Debug)]
struct TotpAuthFormData {
    totp: String,
}

async fn login_totp_auth_submit_handler(
    _: bitsync_routes::PostLoginTotpAuthAction,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    HxRequest(is_hx_request): HxRequest,
    cookie_jar: CookieJar,
    Form(totp_setup_data): Form<TotpAuthFormData>,
) -> impl IntoResponse {
    match verify_totp(
        &auth_data.user,
        &totp_setup_data.totp,
        state.config.auth.jwt_expiration_seconds,
        &state.config.auth.jwt_secret,
    )
    .await
    {
        Ok(jwt) => {
            let cookie_jar = cookie_jar.add(jwt_cookie(jwt));

            (
                cookie_jar,
                redirect_response(is_hx_request, &bitsync_routes::GetFilesHomePage.to_string()),
            )
                .into_response()
        }
        Err(..) => todo!(),
    }
}
