use std::path::PathBuf;

use bitsync_database::{
    database::{transaction::TransactionCommitError, Database, TransactionBeginError},
    entity::User,
    repository,
};
use bitsync_storage::{
    operation::write::{ensure_user_storage_exists, EnsureUserStorageExistsError},
    user_storage::UserStorage,
};

use crate::{
    hash::{hash_password, PasswordHashCreationError},
    jwt::{JwtClaims, LoginState},
};

#[derive(thiserror::Error, Debug)]
#[error("user registration failed")]
pub enum RegistrationError {
    PasswordHash(#[from] PasswordHashCreationError),
    DatabaseQuery(#[from] repository::QueryError),
    DatabaseTransaction(#[from] TransactionCommitError),
    TransactionBegin(#[from] TransactionBeginError),
    UserExists(#[from] UserExists),
    EnsureUserStorageExists(#[from] EnsureUserStorageExistsError),
    Jwt(#[from] crate::jwt::Error),
}

#[derive(thiserror::Error, Debug)]
#[error("the user already exists")]
pub struct UserExists;

pub struct RegistrationResult {
    pub user: User,
    pub jwt: String,
}

pub async fn perform_registration(
    database: &Database,
    storage_root_dir: &PathBuf,
    username: &str,
    password: &str,
    jwt_expiration_seconds: i64,
    jwt_secret: &str,
) -> Result<RegistrationResult, RegistrationError> {
    let mut transaction = database.begin_transaction().await?;

    if let Ok(_user) = repository::user::find_by_username(&mut *transaction, username).await {
        Err(UserExists)?;
    }

    let hashed_password = hash_password(password)?;
    let user = repository::user::create(
        &mut *transaction,
        username,
        &hashed_password,
        &uuid::Uuid::nil().into_bytes(),
    )
    .await?;

    let user_storage = UserStorage {
        user_id: user.id,
        storage_root: storage_root_dir.clone(),
    };

    ensure_user_storage_exists(&user_storage).await?;

    transaction.commit().await?;

    let jwt_expiration = time::OffsetDateTime::now_utc().unix_timestamp() + jwt_expiration_seconds;
    let jwt = JwtClaims {
        sub: user.id,
        exp: jwt_expiration,
        login_state: LoginState::Basic,
    }
    .encode(jwt_secret)?;

    Ok(RegistrationResult { user, jwt })
}
