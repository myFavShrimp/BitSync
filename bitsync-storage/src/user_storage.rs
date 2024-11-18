use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct UserStorage {
    pub user_id: uuid::Uuid,
    pub storage_root: PathBuf,
}

static USER_DATA_DIR: &str = "user";

impl UserStorage {
    pub fn data_directory(&self) -> PathBuf {
        let mut storage_path = self.storage_root.clone();

        storage_path.push(USER_DATA_DIR);
        storage_path.push(self.user_id.to_string());

        storage_path
    }

    pub fn strip_data_dir(&self, path: PathBuf) -> PathBuf {
        path.strip_prefix(self.data_directory())
            .map(|path| path.to_path_buf())
            .unwrap_or(path)
    }
}
