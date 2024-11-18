use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(thiserror::Error, Debug)]
#[error("Password hash creation failed")]
pub struct PasswordHashCreationError(#[from] argon2::password_hash::Error);

pub fn hash_password(password: &str) -> Result<String, PasswordHashCreationError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

#[derive(thiserror::Error, Debug)]
#[error("Password hash verification failed")]
pub struct PasswordHashVerificationError(#[from] argon2::password_hash::Error);

pub fn verify_password_hash(
    hashed_password: &str,
    password: &str,
) -> Result<(), PasswordHashVerificationError> {
    let parsed_hash = PasswordHash::new(hashed_password)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash)?)
}
