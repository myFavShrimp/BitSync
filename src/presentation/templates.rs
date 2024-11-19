use bitsync_core::use_case::user_files::{
    read_user_directory_contents::UserDirectoryContentsResult, upload_user_file::UserFileResult,
};

use super::models::{ParentDirectoryLink, StorageItemPresentation, StorageItemPresentationKind};

pub enum FilesHomeElementId {
    FileUploadForm,
    FileStorageTable,
    FileStorageTableBody,
}

impl FilesHomeElementId {
    pub fn to_str(&self) -> &'static str {
        match self {
            FilesHomeElementId::FileUploadForm => "file_upload_form",
            FilesHomeElementId::FileStorageTable => "file_storage_table",
            FilesHomeElementId::FileStorageTableBody => "file_storage_table_body",
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
pub struct FilesHomeFileStorageTableRowOobSwap {
    dir_item: StorageItemPresentation,
}

impl From<UserFileResult> for FilesHomeFileStorageTableRowOobSwap {
    fn from(value: UserFileResult) -> Self {
        let dir_item = value.storage_item.into();

        FilesHomeFileStorageTableRowOobSwap { dir_item }
    }
}
