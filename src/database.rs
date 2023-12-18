use sqlx::{migrate::MigrateDatabase, PgPool};

pub mod user;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseInitializationError {
    #[error("Could not connect to the database.")]
    Connection(sqlx::Error),
    #[error("Could not apply database migrations.")]
    Creation(sqlx::Error),
    #[error("Could not create the database.")]
    Migration(sqlx::migrate::MigrateError),
}

async fn create_database_if_not_exists(
    database_url: &str,
) -> Result<(), DatabaseInitializationError> {
    if !sqlx::Postgres::database_exists(database_url)
        .await
        .map_err(|error| DatabaseInitializationError::Creation(error))?
    {
        sqlx::Postgres::create_database(database_url)
            .await
            .map_err(|error| DatabaseInitializationError::Creation(error))?
    };

    Ok(())
}

async fn apply_migrations(connection: &PgPool) -> Result<(), DatabaseInitializationError> {
    Ok(sqlx::migrate!()
        .run(connection)
        .await
        .map_err(|error| DatabaseInitializationError::Migration(error))?)
}

pub async fn connect_and_migrate(
    database_url: &str,
) -> Result<PgPool, DatabaseInitializationError> {
    create_database_if_not_exists(database_url).await?;

    let connection = PgPool::connect_lazy(database_url)
        .map_err(|error| DatabaseInitializationError::Connection(error))?;
    apply_migrations(&connection).await?;

    Ok(connection)
}
