use crate::storage::{StorageItem, StorageItemKind};

pub struct StorageItemPresentation {
    pub size: String,
    pub name: String,
    pub icon: String,
}

impl From<StorageItem> for StorageItemPresentation {
    fn from(value: StorageItem) -> Self {
        Self {
            size: format_file_size(value.size),
            name: value.file_name(),
            icon: match value.kind {
                StorageItemKind::Directory => String::from("folder"),
                StorageItemKind::File => String::from("description"),
            },
        }
    }
}

fn format_file_size(bytes: u64) -> String {
    size::Size::from_bytes(bytes)
        .format()
        .with_style(size::Style::Abbreviated)
        .to_string()
}
