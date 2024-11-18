use bitsync_database::database::{ConnectionAcquisitionError, Database};
use bitsync_database::repository;

use crate::jwt::JwtClaims;

use crate::hash::{verify_password_hash, PasswordHashVerificationError};

#[derive(thiserror::Error, Debug)]
#[error("Username or password wrong")]
pub enum LoginError {
    PasswordHashVerification(#[from] PasswordHashVerificationError),
    DatabaseQuery(#[from] repository::QueryError),
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Jwt(#[from] crate::jwt::Error),
}

pub async fn perform_login(
    database: &Database,
    username: &str,
    password: &str,
    jwt_expiration_seconds: i64,
    jwt_secret: &str,
) -> Result<String, LoginError> {
    let mut connection = database.acquire_connection().await?;

    let user = repository::user::find_by_username(&mut *connection, username).await?;

    verify_password_hash(&user.password, password)?;

    let jwt_expiration = time::OffsetDateTime::now_utc().unix_timestamp() + jwt_expiration_seconds;

    Ok(JwtClaims {
        sub: user.id,
        exp: jwt_expiration,
    }
    .encode(jwt_secret)?)
}
