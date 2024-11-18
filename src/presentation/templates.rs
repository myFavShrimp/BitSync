use bitsync_core::use_case::user_files::read_user_directory_contents::UserDirectoryContentsResult;

use super::models::{ParentDirectoryLink, StorageItemPresentation, StorageItemPresentationKind};

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
