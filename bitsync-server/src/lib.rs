use axum::{routing::IntoMakeService, Router};
use bitsync_core::{config::Config, use_case::initialization::InitializationError};
use tower_http::cors::CorsLayer;

mod auth;
mod handler;
mod helper_macro;

pub async fn make_service(config: Config) -> Result<IntoMakeService<Router>, InitializationError> {
    let app_state = bitsync_core::use_case::initialization::initialize(config).await?;

    Ok(handler::create_routes(app_state)
        .await
        .layer(CorsLayer::permissive())
        .into_make_service())
}
