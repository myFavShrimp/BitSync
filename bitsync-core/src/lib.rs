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
    pub public_graphql_api_schema: handler::api::graphql::PublicRoot,
    pub private_graphql_api_schema: handler::api::graphql::PrivateRoot,
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
        public_graphql_api_schema: handler::api::graphql::create_public_root(),
        private_graphql_api_schema: handler::api::graphql::create_private_root(),
    });

    Ok(handler::create_routes(state)
        .await
        .layer(CorsLayer::permissive())
        .into_make_service())
}

pub fn public_graphql_schema_string() -> String {
    handler::api::graphql::create_public_root().sdl()
}
pub fn private_graphql_schema_string() -> String {
    handler::api::graphql::create_private_root().sdl()
}
