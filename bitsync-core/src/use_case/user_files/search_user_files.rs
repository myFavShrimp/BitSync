use std::path::PathBuf;

use bitsync_database::entity::User;
use bitsync_storage::{
    operation::{
        read::{ReadDirContentsError, read_dir_contents},
        write::{EnsureUserStorageExistsError, ensure_user_storage_exists},
    },
    storage_item::{StorageItem, StorageItemKind},
    storage_path::{StoragePath, StoragePathError},
    user_storage::UserStorage,
};
use nucleo_matcher::{
    Matcher, Utf32Str,
    pattern::{AtomKind, CaseMatching, Normalization, Pattern},
};

pub struct SearchResult {
    pub storage_item: StorageItem,
    pub parent_path: String,
    pub score: u32,
}

pub enum SearchUserFilesResult {
    NoSearch,
    Results {
        current_dir_results: Vec<SearchResult>,
        global_results: Vec<SearchResult>,
    },
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to search user files")]
pub enum SearchUserFilesError {
    EnsureUserStorageExists(#[from] EnsureUserStorageExistsError),
    ReadDirContents(#[from] ReadDirContentsError),
    StoragePath(#[from] StoragePathError),
}

#[async_recursion::async_recursion]
async fn collect_all_storage_items(
    path: &StoragePath,
    items: &mut Vec<StorageItem>,
) -> Result<(), ReadDirContentsError> {
    let dir_contents = read_dir_contents(path).await?;

    for item in dir_contents {
        match item.kind {
            StorageItemKind::Directory => {
                collect_all_storage_items(&item.path, items).await?;
                items.push(item);
            }
            StorageItemKind::File => {
                items.push(item);
            }
        }
    }

    Ok(())
}

const SEARCH_RESULT_LIMIT: usize = 10;

pub async fn search_user_files(
    storage_root_dir: &PathBuf,
    query: &str,
    user: &User,
    current_path: Option<&str>,
) -> Result<SearchUserFilesResult, SearchUserFilesError> {
    if query.trim().is_empty() {
        return Ok(SearchUserFilesResult::NoSearch);
    }

    let user_storage = UserStorage {
        user_id: user.id,
        storage_root: storage_root_dir.clone(),
    };

    ensure_user_storage_exists(&user_storage).await?;

    let root_path = StoragePath::new(user_storage, PathBuf::from("/"))?;

    let all_storage_items = {
        let mut all_items = Vec::new();

        collect_all_storage_items(&root_path, &mut all_items).await?;

        all_items
    };

    let pattern = Pattern::new(
        query,
        CaseMatching::Smart,
        Normalization::Smart,
        AtomKind::Fuzzy,
    );
    let mut matcher = Matcher::new(nucleo_matcher::Config::DEFAULT);

    let mut utf32_buffer = Vec::new();

    let mut scored_storage_items: Vec<SearchResult> = all_storage_items
        .into_iter()
        .filter_map(|storage_item| {
            let haystack = storage_item.path.path();

            let score = pattern.score(Utf32Str::new(&haystack, &mut utf32_buffer), &mut matcher)?;

            let parent_path = storage_item
                .path
                .scoped_path
                .parent()
                .map(|parent_path| format!("/{}", parent_path.to_string_lossy()))
                .unwrap_or_else(|| "/".to_owned());

            Some(SearchResult {
                storage_item,
                parent_path,
                score,
            })
        })
        .collect();

    scored_storage_items.sort_by(|a, b| b.score.cmp(&a.score));
    scored_storage_items.truncate(SEARCH_RESULT_LIMIT);

    let (current_dir_results, global_results) = match current_path {
        Some(current_path) => scored_storage_items
            .into_iter()
            .partition(|search_result| search_result.parent_path == current_path),
        None => (Vec::new(), scored_storage_items),
    };

    Ok(SearchUserFilesResult::Results {
        current_dir_results,
        global_results,
    })
}
