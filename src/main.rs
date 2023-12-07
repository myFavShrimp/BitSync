use std::sync::Arc;

use bit_sync::{config::Config, connect_and_migrate, AppState};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config = Config::from_env();
    let address = config.address();

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    let state = Arc::new(AppState {
        postgres_pool: connect_and_migrate(&config.database_url).await?,
        config,
    });

    let app = bit_sync::make_service(state).await;

    Ok(axum::serve(listener, app).await?)
}
