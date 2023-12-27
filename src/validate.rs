#[derive(Debug, thiserror::Error)]
#[error("The path '{0}' is not allowed. Paths must not contain '..'")]
pub struct PathValidationError(String);

pub fn validate_file_path(path: &str) -> Result<(), PathValidationError> {
    if path.contains("..") {
        Err(PathValidationError(path.to_string()))
    } else {
        Ok(())
    }
}

pub fn sanitize_directory_path(path: &str) -> String {
    if !path.ends_with("/") {
        format!("{path}/")
    } else {
        path.to_string()
    }
}
