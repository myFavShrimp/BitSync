use std::sync::Arc;

use axum::{routing::IntoMakeService, Router};
use config::Config;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

mod auth;
pub mod config;
mod database;
mod handler;
mod hash;
mod helper_macro;
mod storage;
mod validate;

pub struct AppState {
    pub config: config::Config,
    pub postgres_pool: PgPool,
}

#[derive(Debug, thiserror::Error)]
pub enum InitializationError {
    #[error("Database initialization failed")]
    Database(#[from] database::DatabaseInitializationError),
}

pub async fn make_service(config: Config) -> Result<IntoMakeService<Router>, InitializationError> {
    let state = Arc::new(AppState {
        postgres_pool: database::connect_and_migrate(&config.database_url).await?,
        config,
    });

    Ok(handler::create_routes(state)
        .await
        .layer(CorsLayer::permissive())
        .into_make_service())
}
