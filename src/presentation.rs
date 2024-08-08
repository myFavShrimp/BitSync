use crate::storage::{StorageItem, StorageItemKind};

pub struct StorageItemPresentation {
    pub size: String,
    pub name: String,
    pub icon: String,
}

impl From<StorageItem> for StorageItemPresentation {
    fn from(value: StorageItem) -> Self {
        Self {
            size: value.size.to_string(),
            name: value.file_name(),
            icon: match value.kind {
                StorageItemKind::Directory => String::from("folder"),
                StorageItemKind::File => String::from("description"),
            },
        }
    }
}
