pub mod config;
mod database;
mod hash;
mod storage;
pub mod use_case;
mod validate;

pub struct AppState {
    pub(crate) config: config::Config,
    pub(crate) postgres_pool: sqlx::PgPool,
}
