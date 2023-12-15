use std::sync::Arc;

use bit_sync::{config::Config, connect_and_migrate, AppState};
use color_eyre::eyre::{self, WrapErr};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let config = Config::from_env();
    let address = config.address();

    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(Config::tracing_level())
            .finish(),
    )
    .wrap_err("Error initializing logging")?;

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .wrap_err(format!("Failed to bind to address '{address}'"))?;

    let state = Arc::new(AppState {
        postgres_pool: connect_and_migrate(&config.database_url).await?,
        config,
    });

    let app = bit_sync::make_service(state).await;

    Ok(axum::serve(listener, app).await?)
}
