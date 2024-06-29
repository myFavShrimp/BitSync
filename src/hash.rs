use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

pub fn verify_password_hash(
    hashed_password: &str,
    password: &str,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hashed_password)?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}
