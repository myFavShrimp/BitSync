use std::sync::Arc;

use axum::{
    extract::State,
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use axum_extra::extract::Form;
use serde::Deserialize;

use crate::{auth::require_logout_middleware, use_case};

use crate::AppState;

use super::routes;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            &routes::GetRegisterPage::handler_route(),
            get(register_page_handler),
        )
        .route(
            &routes::PostRegisterAction::handler_route(),
            post(register_action_handler),
        )
        .route_layer(from_fn_with_state(state.clone(), require_logout_middleware))
        .with_state(state)
}

#[derive(askama::Template, Default)]
#[template(path = "register.html")]
struct Register {
    username: Option<String>,
    error_message: Option<String>,
}

async fn register_page_handler() -> impl IntoResponse {
    Html(Register::default().to_string())
}

#[derive(Deserialize, Clone, Debug)]
struct RegisterActionFormData {
    username: String,
    password: String,
}

async fn register_action_handler(
    State(state): State<Arc<AppState>>,
    Form(registration_data): Form<RegisterActionFormData>,
) -> impl IntoResponse {
    match use_case::register::perform_registration(
        &state,
        &registration_data.username,
        &registration_data.password,
    )
    .await
    {
        Ok(_) => todo!(),
        Err(error) => Register {
            username: Some(registration_data.username),
            error_message: Some(error.to_string()),
        }
        .to_string(),
    }
}
