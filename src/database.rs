use eyre::WrapErr;
use sqlx::{migrate::MigrateDatabase, PgPool};

pub mod user;

const CONNECTION_ERROR_MESSAGE: &str = "Could not connect to the database.";
const MIGRATION_ERROR_MESSAGE: &str = "Could not apply database migrations.";
const CREATION_ERROR_MESSAGE: &str = "Could not create the database.";

async fn create_database_if_not_exists(database_url: &str) -> eyre::Result<()> {
    if !sqlx::Postgres::database_exists(database_url)
        .await
        .wrap_err(CREATION_ERROR_MESSAGE)?
    {
        sqlx::Postgres::create_database(database_url)
            .await
            .wrap_err(CREATION_ERROR_MESSAGE)?
    };

    Ok(())
}

async fn apply_migrations(connection: &PgPool) -> eyre::Result<()> {
    sqlx::migrate!()
        .run(connection)
        .await
        .wrap_err(MIGRATION_ERROR_MESSAGE)
}

pub async fn connect_and_migrate(database_url: &str) -> eyre::Result<PgPool> {
    create_database_if_not_exists(database_url).await?;

    let connection = PgPool::connect_lazy(database_url).wrap_err(CONNECTION_ERROR_MESSAGE)?;
    apply_migrations(&connection).await?;

    Ok(connection)
}
