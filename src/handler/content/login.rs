use std::sync::Arc;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use axum_extra::extract::Form;
use serde::Deserialize;

use crate::use_case;

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
    Form(login_data): Form<LoginActionFormData>,
) -> impl IntoResponse {
    use_case::login::perform_login(&state, login_data.username, login_data.password)
        .await
        .unwrap()
}
