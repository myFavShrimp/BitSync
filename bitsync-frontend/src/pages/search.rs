use bitsync_routes::TypedPath;
use bitsync_storage::storage_item::{StorageItem, StorageItemKind};
use hypertext::prelude::*;

use crate::format_file_size;

pub struct SearchResultItem {
    pub name: String,
    pub path: String,
    pub url: String,
    pub size: String,
    pub is_directory: bool,
}

impl SearchResultItem {
    pub fn new(item: StorageItem, parent_path: String) -> Self {
        let is_directory = matches!(item.kind, StorageItemKind::Directory);

        let url = if is_directory {
            bitsync_routes::GetFilesHomePage
                .with_query_params(bitsync_routes::GetFilesHomePageQueryParameters {
                    path: item.path.path(),
                })
                .to_string()
        } else {
            bitsync_routes::GetUserFileDownload
                .with_query_params(bitsync_routes::GetUserFileDownloadQueryParameters {
                    path: item.path.path(),
                })
                .to_string()
        };

        Self {
            name: item.path.file_name(),
            path: parent_path,
            size: format_file_size(item.size),
            url,
            is_directory,
        }
    }
}

pub struct SearchResults {
    pub current_dir_items: Vec<SearchResultItem>,
    pub global_items: Vec<SearchResultItem>,
}

struct SearchResultList<'a> {
    items: &'a [SearchResultItem],
}

impl Renderable for SearchResultList<'_> {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            @for item in self.items {
                a
                    class=(crate::styles::search_launcher::ClassName::RESULT_ITEM)
                    href=(item.url)
                {
                    @if item.is_directory {
                        span class=(crate::styles::search_launcher::ClassName::RESULT_ICON) {
                            (crate::icons::folder::Folder)
                        }
                    } @else {
                        span class=(crate::styles::search_launcher::ClassName::RESULT_ICON_SECONDARY) {
                            (crate::icons::file_text::FileText)
                        }
                    }
                    div class=(crate::styles::search_launcher::ClassName::RESULT_INFO) {
                        span class=(crate::styles::search_launcher::ClassName::RESULT_NAME) { (item.name) }
                        span class=(crate::styles::search_launcher::ClassName::RESULT_PATH) { (item.path) }
                    }
                }
            }
        }
        .render_to(buffer);
    }
}

impl Renderable for SearchResults {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            @if self.current_dir_items.is_empty() && self.global_items.is_empty() {
                div class=(crate::styles::search_launcher::ClassName::EMPTY) {
                    "No results found"
                }
            } @else {
                @if !self.current_dir_items.is_empty() {
                    div class=(crate::styles::search_launcher::ClassName::CARD) {
                        div class=(crate::styles::search_launcher::ClassName::SECTION_TITLE) {
                            "Current folder"
                        }
                        ((SearchResultList { items: &self.current_dir_items }))
                    }
                }
                @if !self.global_items.is_empty() {
                    div class=(crate::styles::search_launcher::ClassName::CARD) {
                        div class=(crate::styles::search_launcher::ClassName::SECTION_TITLE) {
                            "All files"
                        }
                        ((SearchResultList { items: &self.global_items }))
                    }
                }
            }
        }
        .render_to(buffer);
    }
}
