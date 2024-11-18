use sqlx::{migrate::MigrateDatabase, pool::PoolConnection, PgPool, Postgres, Transaction};

#[derive(Debug, Clone)]
pub struct Database(sqlx::PgPool);

#[derive(thiserror::Error, Debug)]
#[error("Failed to create database")]
pub struct DatabaseCreationError(#[from] sqlx::Error);

#[derive(thiserror::Error, Debug)]
#[error("Failed to connect to database")]
pub struct ConnectionPoolCreationError(#[source] sqlx::Error);

#[derive(thiserror::Error, Debug)]
#[error("Failed to migrate database")]
pub struct DatabaseMigrationApplicationFailure(#[source] sqlx::migrate::MigrateError);

#[derive(thiserror::Error, Debug)]
#[error("Failed to migrate database")]
pub enum DatabaseMigrationError {
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    DatabaseMigrationApplication(#[from] DatabaseMigrationApplicationFailure),
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to connect to database and apply migrations")]
pub enum ConnectAndMigrateError {
    Creation(#[from] DatabaseCreationError),
    Connection(#[from] ConnectionPoolCreationError),
    Migration(#[from] DatabaseMigrationError),
}

impl Database {
    pub async fn connect_and_migrate(connection_url: &str) -> Result<Self, ConnectAndMigrateError> {
        Self::create_database_if_not_exists(connection_url).await?;

        let database =
            Self(PgPool::connect_lazy(connection_url).map_err(ConnectionPoolCreationError)?);

        database.apply_migrations().await?;

        Ok(database)
    }

    async fn create_database_if_not_exists(
        database_url: &str,
    ) -> Result<(), DatabaseCreationError> {
        if !sqlx::Postgres::database_exists(database_url).await? {
            sqlx::Postgres::create_database(database_url).await?;
        };

        Ok(())
    }

    async fn apply_migrations(&self) -> Result<(), DatabaseMigrationError> {
        let mut connection = self.acquire_connection().await?;

        Ok(sqlx::migrate!()
            .run(&mut connection)
            .await
            .map_err(DatabaseMigrationApplicationFailure)?)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Could not begin a new database transaction")]
pub struct TransactionBeginError(#[from] sqlx::Error);

#[derive(thiserror::Error, Debug)]
#[error("Could not acquire a new database connection")]
pub struct ConnectionAcquisitionError(#[from] sqlx::Error);

impl Database {
    pub async fn begin_transaction(
        &self,
    ) -> Result<Transaction<'static, Postgres>, TransactionBeginError> {
        Ok(self.0.begin().await?)
    }

    pub async fn acquire_connection(
        &self,
    ) -> Result<PoolConnection<Postgres>, ConnectionAcquisitionError> {
        Ok(self.0.acquire().await?)
    }
}
