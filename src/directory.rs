use std::path::PathBuf;

use uuid::Uuid;

use crate::config::Config;

static USER_DATA_DIR: &str = "user";

pub fn user_data_directory(config: &Config, user_id: &Uuid) -> PathBuf {
    let mut storage_dir = config.fs_storage_root_dir.clone();
    storage_dir.push(USER_DATA_DIR);
    storage_dir.push(user_id.to_string());

    storage_dir
}
