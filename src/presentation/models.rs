use std::path::PathBuf;

use axum_extra::routing::TypedPath;
use bitsync_storage::storage_item::{StorageItem, StorageItemKind};

use super::format_file_size;

pub struct StorageItemPresentation {
    pub path: String,
    pub size: String,
    pub name: String,
    pub kind: StorageItemPresentationKind,
    pub download_url: String,
    pub delete_url: String,
}

pub enum StorageItemPresentationKind {
    File,
    Directory { url: String },
}

impl StorageItemPresentationKind {
    pub fn icon(&self) -> &'static str {
        match self {
            StorageItemPresentationKind::Directory { .. } => "folder",
            StorageItemPresentationKind::File { .. } => "description",
        }
    }
}

impl From<StorageItem> for StorageItemPresentationKind {
    fn from(value: StorageItem) -> Self {
        match value.kind {
            StorageItemKind::Directory => {
                let directory_url = crate::handler::routes::GetFilesHomePage
                    .with_query_params(crate::handler::routes::GetFilesHomePageQueryParameters {
                        path: value.path.path(),
                    })
                    .to_string();

                StorageItemPresentationKind::Directory { url: directory_url }
            }
            StorageItemKind::File => StorageItemPresentationKind::File,
        }
    }
}

impl From<StorageItem> for StorageItemPresentation {
    fn from(value: StorageItem) -> Self {
        let download_url = crate::handler::routes::GetUserFileDownload
            .with_query_params(crate::handler::routes::GetUserFileDownloadQueryParameters {
                path: value.path.path(),
            })
            .to_string();

        let delete_url = crate::handler::routes::GetUserFileDelete
            .with_query_params(crate::handler::routes::GetUserFileDeleteQueryParameters {
                path: value.path.path(),
            })
            .to_string();

        Self {
            path: value.path.path(),
            size: format_file_size(value.size),
            name: value.path.file_name(),
            kind: StorageItemPresentationKind::from(value),
            download_url,
            delete_url,
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
