use std::path::PathBuf;

use cynic::{coercions::CoercesTo, schema::NamedType, variables::VariableType};
use serde::ser::SerializeSeq;

use crate::api::{http::FileMapper, schema::private as schema, GraphQlVariablesHelper};

cynic::impl_scalar!(PathBuf, schema::StorageItemPath);

#[derive(Clone)]
pub struct UploadFiles(pub Vec<web_sys::File>);

impl CoercesTo<Vec<schema::Upload>> for UploadFiles {}

impl serde::Serialize for UploadFiles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq_serializer = serializer.serialize_seq(Some(self.0.len()))?;

        for _ in 0..(self.0.len()) {
            seq_serializer.serialize_element(&Option::<u8>::None)?;
        }

        seq_serializer.end()
    }
}

impl schema::variable::Variable for UploadFiles {
    const TYPE: VariableType = VariableType::List(&VariableType::Named(schema::Upload::NAME));
}

#[derive(cynic::QueryVariables, Clone)]
pub struct UploadUserFilesMutationVariables {
    pub path: String,
    pub files: UploadFiles,
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
