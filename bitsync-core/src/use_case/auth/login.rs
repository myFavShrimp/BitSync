use bitsync_database::database::{ConnectionAcquisitionError, Database};
use bitsync_database::entity::User;
use bitsync_database::repository;

use crate::jwt::{JwtClaims, LoginState};

use crate::hash::{PasswordHashVerificationError, verify_password_hash};

#[derive(thiserror::Error, Debug)]
#[error("login failed")]
pub enum LoginError {
    PasswordHashVerification(#[from] PasswordHashVerificationError),
    DatabaseQuery(#[from] repository::QueryError),
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    UserNotFound(#[from] UserNotFoundError),
    Jwt(#[from] crate::jwt::Error),
}

#[derive(thiserror::Error, Debug)]
#[error("user not found")]
pub struct UserNotFoundError;

pub struct LoginResult {
    pub user: User,
    pub jwt: String,
}

pub async fn perform_login(
    database: &Database,
    username: &str,
    password: &str,
    user_agent: &str,
    jwt_secret: &str,
) -> Result<LoginResult, LoginError> {
    let mut connection = database.acquire_connection().await?;

    let user = repository::user::find_by_username(&mut *connection, username)
        .await?
        .ok_or(UserNotFoundError)?;

    verify_password_hash(&user.password, password)?;

    let session_platform = super::parse_user_agent_platform(user_agent);
    let session_browser = super::parse_user_agent_browser(user_agent);

    let session = repository::session::create(
        &mut *connection,
        &user.id,
        &session_platform,
        &session_browser,
    )
    .await?;

    let jwt = JwtClaims {
        sub: session.id,
        login_state: LoginState::Basic,
    }
    .encode(jwt_secret)?;

    Ok(LoginResult { user, jwt })
}
