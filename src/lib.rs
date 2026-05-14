use std::sync::Arc;

use axum::{Router, routing::IntoMakeService};
use bitsync_core::{
    config::Config,
    use_case::auth::ensure_admin_bootstrap::{
        AdminBootstrapStatus, EnsureAdminBootstrapError, ensure_admin_bootstrap,
    },
};
use bitsync_database::database::{ConnectAndMigrateError, Database};
use tower_http::cors::CorsLayer;

mod auth;
mod body_limit;
pub mod config;
mod error_report;
mod handler;

pub struct AppState {
    pub(crate) config: Config,
    pub(crate) database: Database,
}

#[derive(thiserror::Error, Debug)]
#[error("failed to initialize application state")]
pub enum ApplicationStateInitializationError {
    ConnectAndMigrate(#[from] ConnectAndMigrateError),
    AdminBootstrap(#[from] EnsureAdminBootstrapError),
}

impl AppState {
    pub async fn from_config(config: Config) -> Result<Self, ApplicationStateInitializationError> {
        let state = Self {
            database: Database::connect_and_migrate(&config.database_url).await?,
            config,
        };

        Ok(state)
    }
}

pub async fn make_service(
    config: Config,
) -> Result<IntoMakeService<Router>, ApplicationStateInitializationError> {
    let app_state = AppState::from_config(config).await?;

    if let AdminBootstrapStatus::RegistrationRequired(token) =
        ensure_admin_bootstrap(&app_state.database).await?
    {
        println!("==========================================================");
        println!("  No admin user found. Use this token to register:");
        println!("  {}", token.id);
        println!("==========================================================");
    }

    Ok(handler::create_routes(Arc::new(app_state))
        .await
        .layer(CorsLayer::permissive())
        .into_make_service())
}
