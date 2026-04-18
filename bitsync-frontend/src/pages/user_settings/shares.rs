use bitsync_core::use_case::user_share::list_shared_paths::SharedPath;
use bitsync_routes::TypedPath;
use bitsync_storage::storage_item::StorageItemKind;
use hypertext::prelude::*;

pub struct SharesTabContent {
    pub shared_paths: Vec<SharedPath>,
}

impl Renderable for SharesTabContent {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            div class=(crate::styles::modal::ClassName::MODAL_BODY) {
                p class=(crate::styles::modal::ClassName::MODAL_DESCRIPTION) {
                    "Files and directories you are currently sharing. Click a path to manage its share links."
                }

                div class=(crate::styles::user_settings_page::ClassName::SHARED_PATH_LIST) {
                    @if self.shared_paths.is_empty() {
                        p class=(crate::styles::user_settings_page::ClassName::SHARED_PATH_EMPTY) {
                            "No shared items yet. Share a file or directory from the file browser."
                        }
                    } @else {
                        @for shared_path in &self.shared_paths {
                            (SharedPathItem {
                                path: shared_path.path.clone(),
                                kind: shared_path.kind.clone(),
                            })
                        }
                    }
                }
            }
        }
        .render_to(buffer);
    }
}

struct SharedPathItem {
    path: String,
    kind: StorageItemKind,
}

impl Renderable for SharedPathItem {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        let share_dialog_url = bitsync_routes::GetUserFileShareDialog
            .with_query_params(bitsync_routes::GetUserFileShareDialogQueryParameters {
                path: self.path.clone(),
            })
            .to_string();

        maud! {
            button
                class=(crate::styles::user_settings_page::ClassName::SHARED_PATH_ITEM)
                data-init=(format!("this.chevronButton = this.querySelector('.{chevron_class}'), this.fetch = fetch('{share_dialog_url}')",
                    chevron_class = crate::styles::user_settings_page::ClassName::SHARED_PATH_CHEVRON,
                ))
                data-on-click__throttle.1s="this.fetch.trigger()"
                data-effect=(format!(
                    "handleButtonLoading(this.chevronButton, this.fetch, '{loading}', 200)",
                    loading = crate::styles::button::ClassName::BUTTON_LOADING,
                ))
            {
                @match self.kind {
                    StorageItemKind::Directory => {
                        (crate::icons::Folder::with_class(crate::styles::user_settings_page::ClassName::SHARED_PATH_ICON))
                    }
                    StorageItemKind::File => {
                        (crate::icons::FileText::with_class(crate::styles::user_settings_page::ClassName::SHARED_PATH_ICON))
                    }
                }

                span class=(crate::styles::user_settings_page::ClassName::SHARED_PATH_NAME) {
                    (self.path)
                }

                span class=(crate::styles::user_settings_page::ClassName::SHARED_PATH_CHEVRON) {
                    div class=(crate::styles::button::ClassName::BUTTON_SPINNER) {}
                    (crate::icons::ChevronRight::default())
                }
            }
        }
        .render_to(buffer);
    }
}
