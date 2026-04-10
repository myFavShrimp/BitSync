use std::path::Path;

use bitsync_database::{
    database::{Database, TransactionBeginError, transaction::TransactionCommitError},
    entity::User,
    repository,
};
use bitsync_storage::{
    operation::write::{EnsureUserStorageExistsError, ensure_user_storage_exists},
    user_storage::UserStorage,
};
use uuid::Uuid;

use crate::{
    hash::{PasswordHashCreationError, hash_password},
    jwt::{JwtClaims, LoginState},
    use_case::auth::InvalidInviteTokenError,
};

#[derive(thiserror::Error, Debug)]
#[error("user registration failed")]
pub enum RegistrationError {
    PasswordHash(#[from] PasswordHashCreationError),
    DatabaseQuery(#[from] repository::QueryError),
    DatabaseTransaction(#[from] TransactionCommitError),
    TransactionBegin(#[from] TransactionBeginError),
    UserExists(#[from] UserExists),
    InvalidInviteTokenError(#[from] InvalidInviteTokenError),
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
    storage_root_dir: &Path,
    username: &str,
    password: &str,
    invite_token_id: &Uuid,
    user_agent: &str,
    jwt_expiration_seconds: i64,
    jwt_secret: &str,
) -> Result<RegistrationResult, RegistrationError> {
    let mut transaction = database.begin_transaction().await?;

    let invite_token =
        repository::invite_token::find_by_id(&mut *transaction, invite_token_id).await?;
    let invite_token = invite_token.ok_or(InvalidInviteTokenError)?;

    if let Ok(_user) = repository::user::find_by_username(&mut *transaction, username).await {
        Err(UserExists)?;
    }

    let hashed_password = hash_password(password)?;
    let user = repository::user::create_with_admin(
        &mut *transaction,
        username,
        &hashed_password,
        &uuid::Uuid::nil().into_bytes(),
        invite_token.is_admin,
    )
    .await?;

    repository::invite_token::delete_by_id(&mut *transaction, &invite_token.id).await?;

    let user_storage = UserStorage {
        user_id: user.id,
        storage_root: storage_root_dir.to_path_buf(),
    };

    ensure_user_storage_exists(&user_storage).await?;

    let session_platform = super::parse_user_agent_platform(user_agent);
    let session_browser = super::parse_user_agent_browser(user_agent);

    let session = repository::session::create(
        &mut *transaction,
        &user.id,
        &session_platform,
        &session_browser,
    )
    .await?;

    transaction.commit().await?;

    let jwt_expiration = time::OffsetDateTime::now_utc().unix_timestamp() + jwt_expiration_seconds;
    let jwt = JwtClaims {
        sub: session.id,
        exp: jwt_expiration,
        login_state: LoginState::Basic,
    }
    .encode(jwt_secret)?;

    Ok(RegistrationResult { user, jwt })
}
