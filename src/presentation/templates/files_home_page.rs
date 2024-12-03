use axum_extra::routing::TypedPath;
use bitsync_core::use_case::user_files::{
    create_directory::DirectoryCreationResult, delete_user_file::UserFileDeletionResult,
    move_user_file::UserFileMoveResult, read_user_directory_contents::UserDirectoryContentsResult,
    upload_user_file::UserFileResult,
};

use crate::{
    handler::routes::{
        PostUserFileDirectoryCreation, PostUserFileDirectoryCreationQueryParameters,
        PostUserFileUpload, PostUserFileUploadQueryParameters,
    },
    presentation::models::{
        ParentDirectoryLink, StorageItemPresentation, StorageItemPresentationKind,
    },
};

pub enum FilesHomePageElementId {
    FileUploadForm,
    DirectoryCreationDialog,
    FileStorageTableWrapper,
}

impl FilesHomePageElementId {
    pub fn to_str(&self) -> &'static str {
        match self {
            FilesHomePageElementId::FileUploadForm => "file_upload_form",
            FilesHomePageElementId::FileStorageTableWrapper => "file_storage_table_wrapper",
            FilesHomePageElementId::DirectoryCreationDialog => "directory_creation_dialog",
        }
    }
}

#[derive(askama::Template)]
#[template(path = "files_home_page.html")]
pub struct FilesHomePage {
    dir_content: Vec<StorageItemPresentation>,
    parent_directory_url: Option<ParentDirectoryLink>,
    file_upload_url: String,
    directory_creation_url: String,
    directory_creation_popover_id: String,
}

impl From<UserDirectoryContentsResult> for FilesHomePage {
    fn from(value: UserDirectoryContentsResult) -> Self {
        let displayable_dir_content = value
            .dir_contents
            .into_iter()
            .map(StorageItemPresentation::from)
            .collect();

        let file_upload_url = PostUserFileUpload
            .with_query_params(PostUserFileUploadQueryParameters {
                path: value.path.path(),
            })
            .to_string();

        let directory_creation_url = PostUserFileDirectoryCreation
            .with_query_params(PostUserFileDirectoryCreationQueryParameters {
                path: value.path.path(),
            })
            .to_string();

        FilesHomePage {
            dir_content: displayable_dir_content,
            parent_directory_url: ParentDirectoryLink::from_child(value.path.scoped_path),
            file_upload_url,
            directory_creation_url,
            directory_creation_popover_id: FilesHomePageElementId::DirectoryCreationDialog
                .to_str()
                .to_owned(),
        }
    }
}

#[derive(askama::Template)]
#[template(path = "files_home_page/file_change_result.html")]
pub struct FilesHomePageChangeResult {
    dir_content: Vec<StorageItemPresentation>,
}

impl From<UserFileResult> for FilesHomePageChangeResult {
    fn from(value: UserFileResult) -> Self {
        let displayable_dir_content = value
            .dir_contents
            .into_iter()
            .map(StorageItemPresentation::from)
            .collect();

        FilesHomePageChangeResult {
            dir_content: displayable_dir_content,
        }
    }
}

impl From<UserFileDeletionResult> for FilesHomePageChangeResult {
    fn from(value: UserFileDeletionResult) -> Self {
        let displayable_dir_content = value
            .dir_contents
            .into_iter()
            .map(StorageItemPresentation::from)
            .collect();

        FilesHomePageChangeResult {
            dir_content: displayable_dir_content,
        }
    }
}

impl From<UserFileMoveResult> for FilesHomePageChangeResult {
    fn from(value: UserFileMoveResult) -> Self {
        let displayable_dir_content = value
            .dir_contents
            .into_iter()
            .map(StorageItemPresentation::from)
            .collect();

        FilesHomePageChangeResult {
            dir_content: displayable_dir_content,
        }
    }
}

impl From<DirectoryCreationResult> for FilesHomePageChangeResult {
    fn from(value: DirectoryCreationResult) -> Self {
        let displayable_dir_content = value
            .dir_contents
            .into_iter()
            .map(StorageItemPresentation::from)
            .collect();

        FilesHomePageChangeResult {
            dir_content: displayable_dir_content,
        }
    }
}
