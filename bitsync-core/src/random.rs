#[derive(thiserror::Error, Debug)]
#[error("failed to generate random value")]
pub struct GenerateRandomBytesError(#[from] getrandom::Error);

pub fn fill_random(buffer: &mut [u8]) -> Result<(), GenerateRandomBytesError> {
    getrandom::fill(buffer)?;

    Ok(())
}
