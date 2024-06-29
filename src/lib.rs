use axum::{routing::IntoMakeService, Router};
use config::Config;
use tower_http::cors::CorsLayer;
use use_case::initialization::InitializationError;

mod auth;
pub mod config;
mod database;
mod handler;
mod hash;
mod jwt;
mod storage;
mod use_case;
mod validate;

pub struct AppState {
    pub(crate) config: config::Config,
    pub(crate) postgres_pool: sqlx::PgPool,
}

pub async fn make_service(config: Config) -> Result<IntoMakeService<Router>, InitializationError> {
    let app_state = use_case::initialization::initialize(config).await?;

    Ok(handler::create_routes(app_state)
        .await
        .layer(CorsLayer::permissive())
        .into_make_service())
}
