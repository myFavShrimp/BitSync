use std::sync::Arc;

use axum::{
    extract::State,
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
    Router,
};
use axum_extra::{extract::Form, routing::RouterExt};
use bitsync_core::use_case::auth::registration::perform_registration;
use serde::Deserialize;

use crate::{
    auth::require_logout_middleware, presentation::templates::register_page::RegisterPage,
};

use crate::AppState;

use super::routes;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_get(register_page_handler)
        .typed_post(register_action_handler)
        .route_layer(from_fn_with_state(state.clone(), require_logout_middleware))
        .with_state(state)
}

async fn register_page_handler(_: routes::GetRegisterPage) -> impl IntoResponse {
    Html(RegisterPage::default().to_string())
}

#[derive(Deserialize, Clone, Debug)]
struct RegisterActionFormData {
    username: String,
    password: String,
}

async fn register_action_handler(
    _: routes::PostRegisterAction,
    State(state): State<Arc<AppState>>,
    Form(registration_data): Form<RegisterActionFormData>,
) -> impl IntoResponse {
    match perform_registration(
        &state.database,
        &state.config.fs_storage_root_dir,
        &registration_data.username,
        &registration_data.password,
    )
    .await
    {
        Ok(_) => todo!(),
        Err(error) => RegisterPage {
            username: Some(registration_data.username),
            error_message: Some(error.to_string()),
        }
        .to_string(),
    }
}
