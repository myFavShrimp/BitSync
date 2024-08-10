use crate::use_case::user_files::UserDirectoryResult;

use super::models::{ParentDirectoryLink, StorageItemPresentation};

#[derive(askama::Template)]
#[template(path = "files_home.html")]
pub struct FilesHome {
    dir_content: Vec<StorageItemPresentation>,
    parent_directory_url: Option<ParentDirectoryLink>,
}

impl From<UserDirectoryResult> for FilesHome {
    fn from(value: UserDirectoryResult) -> Self {
        let displayable_dir_content = value
            .dir_contents
            .into_iter()
            .map(StorageItemPresentation::from)
            .collect();

        FilesHome {
            dir_content: displayable_dir_content,
            parent_directory_url: ParentDirectoryLink::from_child(value.path.scoped_path),
        }
    }
}
