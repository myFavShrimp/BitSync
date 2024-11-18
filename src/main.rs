use bitsync_core::config::Config;
use color_eyre::eyre::{self, WrapErr};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let config = bitsync::config::config_from_env()?;

    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(Config::tracing_level())
            .finish(),
    )
    .wrap_err("Error initializing logging")?;

    let listener = tokio::net::TcpListener::bind(&config.address.0)
        .await
        .wrap_err(format!("Failed to bind to address '{}'", &config.address.0))?;

    let app = bitsync::make_service(config).await?;

    axum::serve(listener, app)
        .await
        .wrap_err("An unrecoverable error occurred")
}
