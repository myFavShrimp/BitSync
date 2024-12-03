use std::path::{Component, Path};

#[derive(Debug, thiserror::Error)]
#[error("The path '{0}' is not allowed. Paths must not contain '..' segments")]
pub struct ScopedPathValidationError(pub String);

pub(crate) fn validate_scoped_path<P: AsRef<Path>>(
    path: P,
) -> Result<(), ScopedPathValidationError> {
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

#[derive(Debug, thiserror::Error)]
pub enum PathIsJustFileNameValidationError {
    #[error("The path '{0}' was expected to be just a file name.")]
    TooManyComponents(String),
    #[error("The path '{0}' was expected to be a file name.")]
    NotAFileName(String),
}

pub fn validate_path_is_just_file_name<P: AsRef<Path>>(
    path: P,
) -> Result<(), PathIsJustFileNameValidationError> {
    let path = path.as_ref();

    let mut directory_components = path.components();

    if directory_components.clone().count() > 1 {
        return Err(PathIsJustFileNameValidationError::TooManyComponents(
            path.to_string_lossy().to_string(),
        ));
    }

    match directory_components.next().unwrap() {
        std::path::Component::Prefix(..)
        | std::path::Component::RootDir
        | std::path::Component::CurDir
        | std::path::Component::ParentDir => Err(PathIsJustFileNameValidationError::NotAFileName(
            path.to_string_lossy().to_string(),
        )),
        std::path::Component::Normal(..) => Ok(()),
    }
}
