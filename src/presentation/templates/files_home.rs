use bitsync_core::use_case::user_files::{
    read_user_directory_contents::UserDirectoryContentsResult, upload_user_file::UserFileResult,
};

use crate::presentation::models::{
    ParentDirectoryLink, StorageItemPresentation, StorageItemPresentationKind,
};

pub enum FilesHomeElementId {
    FileUploadForm,
    FileStorageTableWrapper,
}

impl FilesHomeElementId {
    pub fn to_str(&self) -> &'static str {
        match self {
            FilesHomeElementId::FileUploadForm => "file_upload_form",
            FilesHomeElementId::FileStorageTableWrapper => "file_storage_table_wrapper",
        }
    }
}

#[derive(askama::Template)]
#[template(path = "files_home.html")]
pub struct FilesHome {
    dir_content: Vec<StorageItemPresentation>,
    parent_directory_url: Option<ParentDirectoryLink>,
    file_upload_url: String,
}

impl From<UserDirectoryContentsResult> for FilesHome {
    fn from(value: UserDirectoryContentsResult) -> Self {
        let displayable_dir_content = value
            .dir_contents
            .into_iter()
            .map(StorageItemPresentation::from)
            .collect();

        FilesHome {
            dir_content: displayable_dir_content,
            parent_directory_url: ParentDirectoryLink::from_child(value.path.scoped_path),
            file_upload_url: crate::handler::routes::PostUserFileUpload.to_string(),
        }
    }
}

#[derive(askama::Template)]
#[template(path = "files_home/file_upload_result.html")]
pub struct FilesHomeUploadResult {
    dir_content: Vec<StorageItemPresentation>,
    file_upload_url: String,
}

impl From<UserFileResult> for FilesHomeUploadResult {
    fn from(value: UserFileResult) -> Self {
        let displayable_dir_content = value
            .dir_contents
            .into_iter()
            .map(StorageItemPresentation::from)
            .collect();

        FilesHomeUploadResult {
            dir_content: displayable_dir_content,
            file_upload_url: crate::handler::routes::PostUserFileUpload.to_string(),
        }
    }
}
