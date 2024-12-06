use bitsync_database::{
    database::{transaction::TransactionCommitError, Database, TransactionBeginError},
    entity::User,
    repository,
};

use crate::hash::{
    hash_password, verify_password_hash, PasswordHashCreationError, PasswordHashVerificationError,
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
) -> Result<(), UpdateUserPasswordError> {
    verify_password_hash(&user.password, current_password)?;

    if new_password != new_password_repeated {
        Err(NewPasswordsMismatch)?;
    }

    let hashed_password = hash_password(new_password)?;

    let mut transaction = database.begin_transaction().await?;
    repository::user::update_password(&mut *transaction, &user.id, &hashed_password).await?;

    transaction.commit().await?;

    Ok(())
}
