use std::path::PathBuf;

use crate::api::{http::FileMapper, schema::private as schema, GraphQlVariablesHelper};

pub mod upload_files;

#[derive(cynic::QueryVariables, Clone)]
pub struct UploadUserFilesMutationVariables {
    pub path: String,
    pub files: upload_files::UploadFiles,
}

impl GraphQlVariablesHelper for UploadUserFilesMutationVariables {
    fn file_mapper(&self) -> Option<FileMapper> {
        Some(FileMapper {
            path_prefix: "files",
            files: self.files.0.clone(),
        })
    }
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(schema = "private", graphql_type = "FileItem")]
pub struct FileItem {
    pub path: PathBuf,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(
    schema = "private",
    graphql_type = "Mutation",
    variables = "UploadUserFilesMutationVariables"
)]
pub struct UploadUserFilesMutation {
    #[arguments(path: $path, files: $files)]
    pub upload_user_files: Vec<FileItem>,
}
