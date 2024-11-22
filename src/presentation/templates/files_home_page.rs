use bitsync_core::use_case::user_files::{
    read_user_directory_contents::UserDirectoryContentsResult, upload_user_file::UserFileResult,
};

use crate::presentation::models::{
    ParentDirectoryLink, StorageItemPresentation, StorageItemPresentationKind,
};

pub enum FilesHomePageElementId {
    FileUploadForm,
    FileStorageTableWrapper,
}

impl FilesHomePageElementId {
    pub fn to_str(&self) -> &'static str {
        match self {
            FilesHomePageElementId::FileUploadForm => "file_upload_form",
            FilesHomePageElementId::FileStorageTableWrapper => "file_storage_table_wrapper",
        }
    }
}

#[derive(askama::Template)]
#[template(path = "files_home_page.html")]
pub struct FilesHomePage {
    dir_content: Vec<StorageItemPresentation>,
    parent_directory_url: Option<ParentDirectoryLink>,
    file_upload_url: String,
}

impl From<UserDirectoryContentsResult> for FilesHomePage {
    fn from(value: UserDirectoryContentsResult) -> Self {
        let displayable_dir_content = value
            .dir_contents
            .into_iter()
            .map(StorageItemPresentation::from)
            .collect();

        FilesHomePage {
            dir_content: displayable_dir_content,
            parent_directory_url: ParentDirectoryLink::from_child(value.path.scoped_path),
            file_upload_url: crate::handler::routes::PostUserFileUpload.to_string(),
        }
    }
}

#[derive(askama::Template)]
#[template(path = "files_home_page/file_upload_result.html")]
pub struct FilesHomePageUploadResult {
    dir_content: Vec<StorageItemPresentation>,
    file_upload_url: String,
}

impl From<UserFileResult> for FilesHomePageUploadResult {
    fn from(value: UserFileResult) -> Self {
        let displayable_dir_content = value
            .dir_contents
            .into_iter()
            .map(StorageItemPresentation::from)
            .collect();

        FilesHomePageUploadResult {
            dir_content: displayable_dir_content,
            file_upload_url: crate::handler::routes::PostUserFileUpload.to_string(),
        }
    }
}
