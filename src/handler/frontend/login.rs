use std::sync::Arc;

use axum::{
    extract::State,
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
    Router,
};
use axum_extra::{
    extract::{cookie::SameSite, CookieJar, Form},
    routing::RouterExt,
};
use axum_htmx::HxRequest;
use bitsync_core::use_case::auth::login::perform_login;
use serde::Deserialize;

use crate::{
    auth::require_logout_middleware, handler::redirect_response,
    presentation::templates::login_page::LoginPage,
};

use crate::AppState;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_get(login_page_handler)
        .typed_post(login_action_handler)
        .route_layer(from_fn_with_state(state.clone(), require_logout_middleware))
        .with_state(state)
}

async fn login_page_handler(_: bitsync_routes::GetLoginPage) -> impl IntoResponse {
    Html(LoginPage::default().to_string())
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
        Ok(login_token) => {
            let cookie_jar = cookie_jar.add({
                let mut auth_cookie = axum_extra::extract::cookie::Cookie::new(
                    crate::auth::AUTH_COOKIE_NAME,
                    login_token,
                );
                auth_cookie.set_same_site(SameSite::Strict);

                #[cfg(not(debug_assertions))]
                auth_cookie.set_secure(true);

                auth_cookie
            });

            (
                cookie_jar,
                redirect_response(is_hx_request, &bitsync_routes::GetFilesHomePage.to_string()),
            )
        }
        Err(e) => todo!("login error handling - {:#?}", e),
    }
}
