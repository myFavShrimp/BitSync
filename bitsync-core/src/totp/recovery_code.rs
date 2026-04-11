use std::fmt::Write;

use crate::random::{GenerateRandomBytesError, fill_random};

#[derive(thiserror::Error, Debug)]
#[error("failed to generate recovery code")]
pub enum GenerateRecoveryCodeError {
    GenerateRandomBytes(#[from] GenerateRandomBytesError),
    Fmt(#[from] std::fmt::Error),
}

pub fn generate_recovery_codes_batch() -> Result<[String; 4], GenerateRecoveryCodeError> {
    Ok([
        generate_recovery_code()?,
        generate_recovery_code()?,
        generate_recovery_code()?,
        generate_recovery_code()?,
    ])
}

fn generate_recovery_code() -> Result<String, GenerateRecoveryCodeError> {
    let mut bytes = [0u8; 4];
    fill_random(&mut bytes)?;

    let mut code = String::with_capacity(8);
    for byte in bytes {
        write!(&mut code, "{byte:02x}")?;
    }

    Ok(code)
}
