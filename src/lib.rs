use std::sync::Arc;

use axum::{routing::IntoMakeService, Router};
use sqlx::PgPool;

pub mod config;
mod database;

pub use database::connect_and_migrate;

#[derive(Debug)]
pub struct AppState {
    pub config: config::Config,
    pub postgres_pool: PgPool,
}

pub async fn make_service(state: Arc<AppState>) -> IntoMakeService<Router> {
    Router::new()
        .with_state(state.clone())
        .into_make_service()
}
