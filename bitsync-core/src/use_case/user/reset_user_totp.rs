use bitsync_database::{
    database::{
        ConnectionAcquisitionError, Database, TransactionBeginError,
        transaction::TransactionCommitError,
    },
    entity::User,
    repository::{self, QueryError},
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
#[error("failed to reset user totp")]
pub enum ResetUserTotpError {
    TransactionBegin(#[from] TransactionBeginError),
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    TransactionCommit(#[from] TransactionCommitError),
    Query(#[from] QueryError),
}

pub async fn reset_user_totp(
    database: &Database,
    user_id: &Uuid,
    current_user_id: &Uuid,
) -> Result<Vec<User>, ResetUserTotpError> {
    let mut transaction = database.begin_transaction().await?;

    repository::user::clear_totp_secret(&mut *transaction, user_id).await?;
    repository::totp_recovery_code::delete_all_for_user(&mut *transaction, user_id).await?;
    repository::session::delete_all_by_user_id(&mut *transaction, user_id).await?;

    transaction.commit().await?;

    let mut connection = database.acquire_connection().await?;
    let users = repository::user::find_all_except(&mut *connection, current_user_id).await?;

    Ok(users)
}
