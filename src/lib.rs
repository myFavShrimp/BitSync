use std::sync::{atomic::AtomicU64, Arc};

use axum::{routing::IntoMakeService, Router};
use bitsync_core::config::Config;
use bitsync_database::database::{ConnectAndMigrateError, Database};
use tower_http::cors::CorsLayer;

mod auth;
mod body_limit;
pub mod config;
mod handler;

pub struct AppState {
    pub(crate) config: Config,
    pub(crate) database: Database,
    pub(crate) current_file_upload_limit: AtomicU64,
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to initialize application state")]
pub struct ApplicationStateInitializationError(#[from] ConnectAndMigrateError);

impl AppState {
    pub async fn from_config(config: Config) -> Result<Self, ApplicationStateInitializationError> {
        let state = Self {
            database: Database::connect_and_migrate(&config.database_url).await?,
            config,
            current_file_upload_limit: AtomicU64::new(0),
        };

        Ok(state)
    }
}

pub async fn make_service(
    config: Config,
) -> Result<IntoMakeService<Router>, ApplicationStateInitializationError> {
    let app_state = AppState::from_config(config).await?;

    Ok(handler::create_routes(Arc::new(app_state))
        .await
        .layer(CorsLayer::permissive())
        .into_make_service())
}
