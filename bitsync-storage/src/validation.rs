use std::path::{Component, Path};

#[derive(Debug, thiserror::Error)]
#[error("The path '{0}' is not allowed. Paths must not contain '..' segments")]
pub struct ScopedPathValidationError(pub String);

pub fn validate_scoped_path<P: AsRef<Path>>(path: P) -> Result<(), ScopedPathValidationError> {
    let path = path.as_ref();

    match path
        .components()
        .any(|component| component == Component::ParentDir)
    {
        true => Err(ScopedPathValidationError(
            path.to_string_lossy().to_string(),
        )),
        false => Ok(()),
    }
}
