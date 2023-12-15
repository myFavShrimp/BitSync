use std::sync::Arc;

use axum::{routing::IntoMakeService, Router};
use sqlx::PgPool;

pub mod config;
mod database;
mod handler;

pub use database::connect_and_migrate;

#[derive(Debug)]
pub struct AppState {
    pub config: config::Config,
    pub postgres_pool: PgPool,
}

pub async fn make_service(state: Arc<AppState>) -> IntoMakeService<Router> {
    handler::create_routes(state).await.into_make_service()
}
