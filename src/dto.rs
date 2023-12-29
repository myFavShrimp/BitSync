use std::{fs::Metadata, path::Path};

use time::OffsetDateTime;

#[derive(async_graphql::SimpleObject)]
pub struct DirectoryEntry {
    path: String,
    size: u64,
    updated_at: OffsetDateTime,
}

impl DirectoryEntry {
    pub fn from_metadata<P: AsRef<str>>(
        scoped_path: P,
        metadata: Metadata,
    ) -> Result<Self, std::io::Error> {
        Ok(Self {
            path: scoped_path.as_ref().to_string(),
            size: metadata.len(),
            updated_at: metadata.modified()?.into(),
        })
    }

    pub async fn from_dir_entry(
        storage_root: &Path,
        value: tokio::fs::DirEntry,
    ) -> Result<Self, std::io::Error> {
        let metadata = value.metadata().await?;
        let path = value.path().to_string_lossy().to_string();
        let path = path
            .strip_prefix(&storage_root.to_string_lossy().to_string())
            .unwrap_or(&path)
            .to_string();

        Self::from_metadata(path, metadata)
    }
}
