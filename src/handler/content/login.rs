use std::sync::Arc;

use axum::{
    extract::State,
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use axum_extra::extract::{cookie::SameSite, CookieJar, Form};
use serde::Deserialize;

use crate::{auth::require_logout_middleware, use_case};

use crate::AppState;

use super::routes;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            &routes::GetLoginPage::handler_route(),
            get(login_page_handler),
        )
        .route(
            &routes::PostLoginAction::handler_route(),
            post(login_action_handler),
        )
        .route_layer(from_fn_with_state(state.clone(), require_logout_middleware))
        .with_state(state)
}

#[derive(askama::Template)]
#[template(path = "login.html")]
struct Login;

async fn login_page_handler() -> impl IntoResponse {
    Html(Login.to_string())
}

#[derive(Deserialize, Clone, Debug)]
struct LoginActionFormData {
    username: String,
    password: String,
}

async fn login_action_handler(
    State(state): State<Arc<AppState>>,
    cookie_jar: CookieJar,
    Form(login_data): Form<LoginActionFormData>,
) -> impl IntoResponse {
    match use_case::login::perform_login(&state, login_data.username, login_data.password).await {
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
                [("HX-Redirect", routes::FilesHome::route_path())],
            )
        }
        Err(_) => todo!(),
    }
}
