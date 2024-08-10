use axum_extra::routing::TypedPath;

use crate::storage::{StorageItem, StorageItemKind};

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

fn format_file_size(bytes: u64) -> String {
    size::Size::from_bytes(bytes)
        .format()
        .with_style(size::Style::Abbreviated)
        .to_string()
}
