use std::{io::Read, path::PathBuf};

use async_graphql::Upload;

use crate::{directory::user_data_directory, dto::File, handler::api::graphql::PrivateContext};

#[derive(thiserror::Error, Debug)]
#[error("An unexpected error occurred")]
pub enum UserFileUploadError {
    Context(async_graphql::Error),
    Opendal(#[from] opendal::Error),
}

pub async fn upload_user_file<'context>(
    ctx: &async_graphql::Context<'context>,
    path: &str,
    mut files: Vec<Upload>,
) -> Result<Vec<File>, UserFileUploadError> {
    let context = ctx
        .data::<PrivateContext>()
        .map_err(|error| UserFileUploadError::Context(error))?;

    let user_directory = user_data_directory(&context.app_state.config, &context.current_user.id);
    let mut fs_storage_dir = user_directory.clone();
    fs_storage_dir.push(path.strip_prefix("/").unwrap());

    let mut fs_builder = opendal::services::Fs::default();
    fs_builder.root(dbg!(fs_storage_dir).to_str().unwrap());

    let op = opendal::Operator::new(fs_builder)?.finish();

    let files: Result<Vec<async_graphql::UploadValue>, std::io::Error> =
        files.iter_mut().map(|file| file.value(ctx)).collect();

    let mut result = Vec::new();
    for file in files.unwrap() {
        let file_name = &file.filename;
        let mut file_content = file.content;

        let mut data = Vec::new();
        file_content.read_to_end(&mut data).unwrap();

        op.write(&file_name, data).await.unwrap();

        let mut file_path = PathBuf::from(path);
        file_path.push(path);
        file_path.push(file_name);
        result.push(
            (
                file_path.to_str().unwrap().to_owned(),
                op.stat(&file_name).await?,
            )
                .into(),
        );
    }

    Ok(result)
}
