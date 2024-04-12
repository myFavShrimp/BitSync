use std::sync::Arc;

use crate::{
    config::Config,
    database::{self, DatabaseInitializationError},
    AppState,
};

#[derive(Debug, thiserror::Error)]
#[error("Initialization failed")]
pub struct InitializationError(#[from] DatabaseInitializationError);

pub async fn initialize(config: Config) -> Result<Arc<AppState>, DatabaseInitializationError> {
    let state = Arc::new(AppState {
        postgres_pool: database::connect_and_migrate(&config.database_url).await?,
        config,
    });

    Ok(state)
}
