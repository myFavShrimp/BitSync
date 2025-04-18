use std::sync::Arc;

use axum::Router;

use crate::AppState;

mod files_home;
mod login;
mod logout;
mod register;
mod user_file;
mod user_settings;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(user_settings::create_routes(state.clone()).await)
        .merge(files_home::create_routes(state.clone()).await)
        .merge(login::create_routes(state.clone()).await)
        .merge(register::create_routes(state.clone()).await)
        .merge(logout::create_routes(state.clone()).await)
        .merge(user_file::create_routes(state).await)
}
