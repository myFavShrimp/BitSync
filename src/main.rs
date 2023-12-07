use bit_sync::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let address = config.address();

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    let app = axum::Router::new();

    axum::serve(listener, app).await.unwrap();
}
