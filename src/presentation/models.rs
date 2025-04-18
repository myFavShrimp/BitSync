use std::path::PathBuf;

use axum_extra::routing::TypedPath;
use bitsync_storage::storage_item::{StorageItem, StorageItemKind};

use super::format_file_size;

pub struct StorageItemPresentation {
    pub size: String,
    pub name: String,
    pub path: String,
    pub kind: StorageItemPresentationKind,
    pub actions_popover_id: String,
    pub actions_move_popover_id: String,
    pub download_url: String,
    pub move_url: String,
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
                let directory_url = bitsync_routes::GetFilesHomePage
                    .with_query_params(bitsync_routes::GetFilesHomePageQueryParameters {
                        path: value.path.path(),
                    })
                    .to_string();

                StorageItemPresentationKind::Directory { url: directory_url }
            }
            StorageItemKind::File => StorageItemPresentationKind::File,
        }
    }
}

static ACTIONS_POPOVER_ID_PART: &str = "actions-popover";

impl From<StorageItem> for StorageItemPresentation {
    fn from(value: StorageItem) -> Self {
        let path = value.path.path();

        let download_url = bitsync_routes::GetUserFileDownload
            .with_query_params(bitsync_routes::GetUserFileDownloadQueryParameters {
                path: value.path.path(),
            })
            .to_string();

        let move_url = bitsync_routes::PostUserFileMove
            .with_query_params(bitsync_routes::PostUserFileMoveQueryParameters {
                path: value.path.path(),
            })
            .to_string();

        let delete_url = bitsync_routes::GetUserFileDelete
            .with_query_params(bitsync_routes::GetUserFileDeleteQueryParameters {
                path: value.path.path(),
            })
            .to_string();

        let actions_popover_id = format!("{}-{ACTIONS_POPOVER_ID_PART}", value.path.file_name());
        let actions_move_popover_id =
            format!("{}-{ACTIONS_POPOVER_ID_PART}-move", value.path.file_name());

        Self {
            size: format_file_size(value.size),
            name: value.path.file_name(),
            path,
            kind: StorageItemPresentationKind::from(value),
            actions_popover_id,
            actions_move_popover_id,
            download_url,
            move_url,
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

                let parent_directory_url = bitsync_routes::GetFilesHomePage
                    .with_query_params(bitsync_routes::GetFilesHomePageQueryParameters {
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
