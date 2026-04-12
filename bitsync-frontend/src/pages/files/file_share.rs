use bitsync_database::entity::UserShare;
use bitsync_routes::TypedPath;
use hypertext::prelude::*;

use crate::{Component, error_banner::OptionalErrorBanner};

use super::FilesHomePageElementId;

pub enum ShareDisplayError {
    InternalServerError,
}

impl ShareDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::InternalServerError => "An internal server error occurred",
        }
    }
}

static SHARE_DIALOG_BODY_ID: &str = "share-dialog-body";

pub struct FileShareDialog {
    pub user_shares: Vec<UserShare>,
    pub item_path: String,
    pub create_user_share_url: String,
    pub delete_all_user_shares_url: String,
}

impl Component for FileShareDialog {
    fn id(&self) -> String {
        FilesHomePageElementId::FileShareDialog.to_str().to_owned()
    }
}

impl Renderable for FileShareDialog {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            dialog
                class=(crate::styles::modal::ClassName::MODAL)
                id=(self.id())
                data-init="this.showModal()"
                onclick="if (event.target === this) closeClosestDialogAndRemoveElement(this)"
            {
                div class=(crate::styles::modal::ClassName::MODAL_HEADER) {
                    h2 class=(crate::styles::modal::ClassName::MODAL_TITLE) { "Share Item" }

                    button
                        class=(crate::styles::modal::ClassName::MODAL_CLOSE)
                        onclick="closeClosestDialogAndRemoveElement(this)"
                    {
                        (crate::icons::x::X)
                    }
                }
                (ShareDialogBody {
                    user_shares: self.user_shares.clone(),
                    item_path: self.item_path.clone(),
                    create_user_share_url: self.create_user_share_url.clone(),
                    delete_all_user_shares_url: self.delete_all_user_shares_url.clone(),
                    error: None,
                })
            }
        }
        .render_to(buffer);
    }
}

pub struct ShareDialogBody {
    pub user_shares: Vec<UserShare>,
    pub item_path: String,
    pub create_user_share_url: String,
    pub delete_all_user_shares_url: String,
    pub error: Option<ShareDisplayError>,
}

impl Component for ShareDialogBody {
    fn id(&self) -> String {
        SHARE_DIALOG_BODY_ID.to_owned()
    }
}

impl Renderable for ShareDialogBody {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            div id=(self.id()) {
                div class=(crate::styles::modal::ClassName::MODAL_BODY) {
                    p class=(crate::styles::modal::ClassName::MODAL_DESCRIPTION) {
                        "Share links for this item. Anyone with a share link can access this file."
                    }

                    OptionalErrorBanner message=(self.error.as_ref().map(|error| error.message().to_owned()));

                    (ShareList {
                        user_shares: self.user_shares.clone(),
                        item_path: self.item_path.clone(),
                    })

                    div class=(crate::styles::files_home_page::ClassName::SHARE_ACTIONS) {
                        button
                            class=(
                                crate::styles::button::ClassName::BUTTON, " ",
                                crate::styles::button::ClassName::BUTTON_PRIMARY,
                            )
                            data-init=(format!(
                                "this.fetch = fetch('{}', {{ method: 'POST' }})",
                                self.create_user_share_url,
                            ))
                            data-on-click__throttle.1s="this.fetch.trigger()"
                            data-effect=(format!(
                                "handleButtonLoading(this, this.fetch, '{loading}', 200)",
                                loading = crate::styles::button::ClassName::BUTTON_LOADING,
                            ))
                        {
                            div class=(crate::styles::button::ClassName::BUTTON_SPINNER) {}
                            (crate::icons::plus::Plus)
                            "Add Share"
                        }

                        @if !self.user_shares.is_empty() {
                            button
                                class=(
                                    crate::styles::button::ClassName::BUTTON, " ",
                                    crate::styles::button::ClassName::BUTTON_DANGER,
                                )
                                data-init=(format!(
                                    "this.fetch = fetch('{}', {{ method: 'POST' }})",
                                    self.delete_all_user_shares_url,
                                ))
                                data-on-click__throttle.1s="this.fetch.trigger()"
                                data-effect=(format!(
                                    "handleButtonLoading(this, this.fetch, '{loading}', 200)",
                                    loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                ))
                            {
                                div class=(crate::styles::button::ClassName::BUTTON_SPINNER) {}
                                "Remove All"
                            }
                        }
                    }
                }
            }
        }
        .render_to(buffer);
    }
}

struct ShareList {
    user_shares: Vec<UserShare>,
    item_path: String,
}

impl Renderable for ShareList {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            div class=(crate::styles::files_home_page::ClassName::SHARE_LIST) {
                @if self.user_shares.is_empty() {
                    p class=(crate::styles::files_home_page::ClassName::SHARE_EMPTY) {
                        "No shares yet. Add one to create a share link."
                    }
                } @else {
                    @for user_share in &self.user_shares {
                        (ShareItem {
                            user_share: user_share.clone(),
                            item_path: self.item_path.clone(),
                        })
                    }
                }
            }
        }
        .render_to(buffer);
    }
}

struct ShareItem {
    user_share: UserShare,
    item_path: String,
}

impl Renderable for ShareItem {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        let token = self.user_share.id.to_string();

        let delete_url = bitsync_routes::PostUserFileShareDelete {
            user_share_id: self.user_share.id,
        }
        .with_query_params(bitsync_routes::PostUserFileShareDeleteQueryParameters {
            path: self.item_path.clone(),
        })
        .to_string();

        maud! {
            div class=(crate::styles::files_home_page::ClassName::SHARE_ITEM) {
                div class=(crate::styles::files_home_page::ClassName::SHARE_TOKEN_WRAPPER) {
                    pre
                        class=(crate::styles::files_home_page::ClassName::SHARE_ID)
                        data-init=(format!(
                            "this.updateOverflow = createHorizontalOverflowHandler(this, '{overflow_left}', '{overflow_right}'), this.updateOverflow()",
                            overflow_left = crate::styles::files_home_page::ClassName::OVERFLOW_LEFT,
                            overflow_right = crate::styles::files_home_page::ClassName::OVERFLOW_RIGHT,
                        ))
                        data-on-scroll="this.updateOverflow()"
                    {
                        (token)
                    }
                }

                div class=(crate::styles::files_home_page::ClassName::SHARE_ACTIONS_GROUP) {
                    button
                        type="button"
                        class=(crate::styles::files_home_page::ClassName::SHARE_COPY)
                        title="Copy share id"
                        data-init=(format!(
                            "this.shareId = '{token}'"
                        ))
                        data-on-click=(format!(
                            "navigator.clipboard.writeText(this.shareId), this.classList.add('{copied}'), setTimeout(() => this.classList.remove('{copied}'), 1200)",
                            copied = crate::styles::files_home_page::ClassName::COPIED,
                        ))
                    {
                        span class=(crate::styles::files_home_page::ClassName::SHARE_COPY_ICON_DEFAULT) {
                            (crate::icons::link::Link)
                        }
                        span class=(crate::styles::files_home_page::ClassName::SHARE_COPY_ICON_COPIED) {
                            (crate::icons::check::Check)
                        }
                    }

                    button
                        type="button"
                        class=(crate::styles::files_home_page::ClassName::SHARE_REVOKE)
                        title="Remove share"
                        data-init=(format!(
                            "this.fetch = fetch('{}', {{ method: 'POST' }})",
                            delete_url,
                        ))
                        data-on-click__throttle.1s="this.fetch.trigger()"
                        data-effect=(format!(
                            "handleButtonLoading(this, this.fetch, '{loading}', 200)",
                            loading = crate::styles::button::ClassName::BUTTON_LOADING,
                        ))
                    {
                        div class=(crate::styles::button::ClassName::BUTTON_SPINNER) {}
                        (crate::icons::circle_x::CircleX)
                    }
                }
            }
        }
        .render_to(buffer);
    }
}
