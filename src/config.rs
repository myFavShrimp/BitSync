use std::net::SocketAddr;

use serde::Deserialize;
use serde_env::from_env;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    host_name: String,
    port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_seconds: i64,
}

impl Config {
    pub fn tracing_level() -> tracing::level_filters::LevelFilter {
        #[cfg(debug_assertions)]
        return tracing::level_filters::LevelFilter::TRACE;
        #[cfg(not(debug_assertions))]
        return tracing::level_filters::LevelFilter::INFO;
    }

    pub fn address(&self) -> SocketAddr {
        format!("{}:{}", self.host_name, self.port)
            .parse()
            .expect("valid host address")
    }

    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        from_env::<Config>().expect("load config")
    }
}
