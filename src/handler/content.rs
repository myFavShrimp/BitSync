use std::sync::Arc;

use axum::Router;

use crate::AppState;

use super::routes;

mod files_home;
mod login;
mod register;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(files_home::create_routes(state.clone()).await)
        .merge(login::create_routes(state.clone()).await)
        .merge(register::create_routes(state).await)
}
