use bitsync_database::{
    database::{Database, TransactionBeginError, transaction::TransactionCommitError},
    entity::User,
    repository,
};
use uuid::Uuid;

use crate::hash::{
    PasswordHashCreationError, PasswordHashVerificationError, hash_password, verify_password_hash,
};

#[derive(thiserror::Error, Debug)]
#[error("Failed to update user password")]
pub enum UpdateUserPasswordError {
    PasswordHashVerification(#[from] PasswordHashVerificationError),
    PasswordHash(#[from] PasswordHashCreationError),
    Database(#[from] repository::QueryError),
    TransactionBegin(#[from] TransactionBeginError),
    PasswordsMismatch(#[from] NewPasswordsMismatch),
    DatabaseTransaction(#[from] TransactionCommitError),
}

#[derive(thiserror::Error, Debug)]
#[error("The new passwords do not match")]
pub struct NewPasswordsMismatch;

pub async fn update_user_password(
    database: &Database,
    user: &User,
    current_password: &str,
    new_password: &str,
    new_password_repeated: &str,
    current_session_id: &Uuid,
) -> Result<(), UpdateUserPasswordError> {
    verify_password_hash(&user.password, current_password)?;

    if new_password != new_password_repeated {
        Err(NewPasswordsMismatch)?;
    }

    let hashed_password = hash_password(new_password)?;

    let mut transaction = database.begin_transaction().await?;
    repository::user::update_password(&mut *transaction, &user.id, &hashed_password).await?;
    repository::session::delete_all_by_user_id_except(
        &mut *transaction,
        &user.id,
        current_session_id,
    )
    .await?;

    transaction.commit().await?;

    Ok(())
}
