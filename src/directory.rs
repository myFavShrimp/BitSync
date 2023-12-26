use std::path::PathBuf;

use uuid::Uuid;

static USER_DATA_DIR: &str = "user";

pub fn user_data_directory(mut storage_root: PathBuf, user_id: &Uuid) -> PathBuf {
    storage_root.push(USER_DATA_DIR);
    storage_root.push(user_id.to_string());

    storage_root
}
