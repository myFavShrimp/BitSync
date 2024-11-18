use bitsync_core::config::Config;
use serde_env::from_env;

#[derive(thiserror::Error, Debug)]
#[error("Failed to read config from environment")]
pub struct ConfigFromEnvError(#[from] serde_env::Error);

pub fn config_from_env() -> Result<Config, ConfigFromEnvError> {
    dotenv::dotenv().ok();

    Ok(from_env::<Config>()?)
}
