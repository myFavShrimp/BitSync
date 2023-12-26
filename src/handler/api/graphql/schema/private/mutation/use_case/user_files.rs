use std::{io::Read, path::PathBuf};

use async_graphql::Upload;

use crate::{
    directory::user_data_directory,
    dto::File,
    handler::api::graphql::PrivateContext,
    validate::{validate_file_path, PathValidationError},
};

#[derive(thiserror::Error, Debug)]
pub enum UserFileUploadError {
    #[error("An unexpected error occurred")]
    Context(async_graphql::Error),
    #[error("An unexpected error occurred")]
    Opendal(opendal::Error),
    #[error(transparent)]
    PathValidation(#[from] PathValidationError),
    #[error("Could not read the file data")]
    FileUploadRead(std::io::Error),
    #[error("Could not read the data of '{file_name}'")]
    FileReader {
        #[source]
        source: std::io::Error,
        file_name: String,
    },
    #[error("Could not write the data of '{file_path}'")]
    FileWriter {
        #[source]
        source: opendal::Error,
        file_path: String,
    },
}

pub async fn upload_user_file<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
    mut files: Vec<Upload>,
) -> Result<Vec<File>, UserFileUploadError> {
    validate_file_path(path)?;

    let context = ctx
        .data::<PrivateContext>()
        .map_err(UserFileUploadError::Context)?;

    let user_directory = user_data_directory(
        context.app_state.config.fs_storage_root_dir.clone(),
        &context.current_user.id,
    );
    let mut fs_storage_dir = user_directory.clone();
    fs_storage_dir.push(path.strip_prefix('/').unwrap_or(path));

    let mut fs_builder = opendal::services::Fs::default();
    fs_builder.root(fs_storage_dir.to_str().expect("Path is valid UTF-8"));

    let op = opendal::Operator::new(fs_builder)
        .map_err(UserFileUploadError::Opendal)?
        .finish();

    let files: Result<Vec<async_graphql::UploadValue>, std::io::Error> =
        files.iter_mut().map(|file| file.value(ctx)).collect();

    let mut result = Vec::new();
    for file in files.map_err(UserFileUploadError::FileUploadRead)? {
        let file_name = &file.filename;
        let mut file_content = file.content;

        validate_file_path(file_name)?;

        let mut data = Vec::new();
        file_content
            .read_to_end(&mut data)
            .map_err(|error| UserFileUploadError::FileReader {
                source: error,
                file_name: file_name.to_string(),
            })?;

        op.write(file_name, data)
            .await
            .map_err(|error| UserFileUploadError::FileWriter {
                source: error,
                file_path: format!("{path}/{file_name}"),
            })?;

        let mut file_path = PathBuf::from(path);
        file_path.push(path);
        file_path.push(file_name);
        result.push(
            (
                file_path.to_str().expect("Path is valid UTF-8").to_owned(),
                op.stat(file_name)
                    .await
                    .map_err(UserFileUploadError::Opendal)?,
            )
                .into(),
        );
    }

    Ok(result)
}
