pub mod totp_recovery_code;
pub mod user;

#[derive(thiserror::Error, Debug)]
#[error("A database query resulted in an error")]
pub struct QueryError(#[from] sqlx::Error);
