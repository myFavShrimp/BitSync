use bitsync_database::database::{ConnectionAcquisitionError, Database};
use bitsync_database::entity::User;
use bitsync_database::repository;

use crate::jwt::{JwtClaims, LoginState};

use crate::hash::{PasswordHashVerificationError, verify_password_hash};

#[derive(thiserror::Error, Debug)]
#[error("Username or password wrong")]
pub enum LoginError {
    PasswordHashVerification(#[from] PasswordHashVerificationError),
    DatabaseQuery(#[from] repository::QueryError),
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Jwt(#[from] crate::jwt::Error),
}

pub struct LoginResult {
    pub user: User,
    pub jwt: String,
}

pub async fn perform_login(
    database: &Database,
    username: &str,
    password: &str,
    platform: &str,
    jwt_expiration_seconds: i64,
    jwt_secret: &str,
) -> Result<LoginResult, LoginError> {
    let mut connection = database.acquire_connection().await?;

    let user = repository::user::find_by_username(&mut *connection, username).await?;

    verify_password_hash(&user.password, password)?;

    let session_platform = crate::use_case::auth::parse_navigator_platform(platform);
    let session =
        repository::session::create(&mut *connection, &user.id, &session_platform).await?;

    let jwt_expiration = time::OffsetDateTime::now_utc().unix_timestamp() + jwt_expiration_seconds;
    let jwt = JwtClaims {
        sub: session.id,
        exp: jwt_expiration,
        login_state: LoginState::Basic,
    }
    .encode(jwt_secret)?;

    Ok(LoginResult { user, jwt })
}
