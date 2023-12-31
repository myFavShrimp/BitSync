#[derive(Debug, thiserror::Error)]
#[error("The path '{0}' is not allowed. Paths must not contain '..' segments")]
pub struct PathValidationError(String);

pub fn validate_file_path(path: &str) -> Result<(), PathValidationError> {
    if path == ".." || path.contains("../") || path.contains("/..") {
        Err(PathValidationError(path.to_string()))
    } else {
        Ok(())
    }
}
