use std::path::PathBuf;

use axum_extra::routing::TypedPath;

use crate::storage::{StorageItem, StorageItemKind};

use super::format_file_size;

pub struct StorageItemPresentation {
    pub path: String,
    pub size: String,
    pub name: String,
    pub icon: String,
    pub directory_url: Option<String>,
}

impl From<StorageItem> for StorageItemPresentation {
    fn from(value: StorageItem) -> Self {
        let directory_url = match value.kind {
            StorageItemKind::Directory => Some(
                crate::handler::routes::GetFilesHomePage
                    .with_query_params(crate::handler::routes::GetFilesHomePageQueryParameters {
                        path: value.path(),
                    })
                    .to_string(),
            ),
            StorageItemKind::File => None,
        };

        Self {
            path: value.path(),
            size: format_file_size(value.size),
            name: value.file_name(),
            icon: match value.kind {
                StorageItemKind::Directory => String::from("folder"),
                StorageItemKind::File => String::from("description"),
            },
            directory_url,
        }
    }
}

pub struct ParentDirectoryLink {
    pub parent_directory_url: String,
    pub current_directory_name: String,
}

impl ParentDirectoryLink {
    pub fn from_child(value: PathBuf) -> Option<Self> {
        match value.parent() {
            Some(parent_directory) => {
                let current_directory_name = value.to_string_lossy().to_string();
                let parent_directory_string = parent_directory.to_string_lossy().to_string();

                let parent_directory_url = crate::handler::routes::GetFilesHomePage
                    .with_query_params(crate::handler::routes::GetFilesHomePageQueryParameters {
                        path: parent_directory_string,
                    })
                    .to_string();

                Some(Self {
                    parent_directory_url,
                    current_directory_name,
                })
            }
            None => None,
        }
    }
}
