use bitsync_database::entity::User;
use hypertext::prelude::*;

use crate::Component;

static USER_LIST_ID: &str = "user-list";

pub struct UsersTabContent {
    pub users: Vec<User>,
}

impl Renderable for UsersTabContent {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            div class=(crate::styles::modal::ClassName::MODAL_BODY) {
                p class=(crate::styles::modal::ClassName::MODAL_DESCRIPTION) {
                    "All registered users on this instance."
                }

                (UserList {
                    users: self.users.clone(),
                })
            }
        }
        .render_to(buffer);
    }
}

pub struct UserList {
    pub users: Vec<User>,
}

impl Component for UserList {
    fn id(&self) -> String {
        USER_LIST_ID.to_owned()
    }
}

impl Renderable for UserList {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            div
                id=(self.id())
                class=(crate::styles::user_settings_page::ClassName::USER_LIST)
            {
                @if self.users.is_empty() {
                    p class=(crate::styles::empty_state::ClassName::EMPTY_STATE) {
                        "No other users on this instance."
                    }
                }
                @for user in &self.users {
                    @let popover_id = format!("user-actions-{}", user.id);

                    div class=(crate::styles::user_settings_page::ClassName::USER_ITEM) {
                        span class=(crate::styles::user_settings_page::ClassName::USER_NAME) {
                            (user.username)
                        }

                        div class=(crate::styles::user_settings_page::ClassName::USER_ACTIONS_GROUP) {
                            @if user.is_admin {
                                span class=(
                                    crate::styles::badge::ClassName::BADGE, " ",
                                    crate::styles::badge::ClassName::BADGE_DANGER,
                                ) {
                                    "Admin"
                                }
                            }
                            @if user.is_suspended {
                                span class=(
                                    crate::styles::badge::ClassName::BADGE, " ",
                                    crate::styles::badge::ClassName::BADGE_WARNING,
                                ) {
                                    "Suspended"
                                }
                            }

                            button
                                class=(crate::styles::button::ClassName::ICON_BUTTON)
                                popovertarget=(popover_id)
                                title="User actions"
                            {
                                div class=(crate::styles::button::ClassName::BUTTON_SPINNER) {}

                                (crate::icons::EllipsisVertical::default())
                            }
                        }

                        div
                            id=(popover_id)
                            class=(
                                crate::styles::context_menu::ClassName::CONTEXT_MENU, " ",
                                crate::styles::context_menu::ClassName::ANCHOR_TOP_RIGHT,
                            )
                            popover
                        {
                            @if user.is_admin {
                                button
                                    class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM)
                                    data-init=(format!("this.triggerButton = getPopoverTrigger(this), this.fetch = fetch('{}')", bitsync_routes::GetRevokeAdminDialog { user_id: user.id }))
                                    data-on-click="closeClosestPopover(this), this.fetch.trigger()"
                                    data-effect=(format!(
                                        "handleButtonLoading(this.triggerButton, this.fetch, '{loading}')",
                                        loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                    ))
                                {
                                    span { "Revoke Admin" }
                                }
                            } @else {
                                button
                                    class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM)
                                    data-init=(format!("this.triggerButton = getPopoverTrigger(this), this.fetch = fetch('{}')", bitsync_routes::GetMakeAdminDialog { user_id: user.id }))
                                    data-on-click="closeClosestPopover(this), this.fetch.trigger()"
                                    data-effect=(format!(
                                        "handleButtonLoading(this.triggerButton, this.fetch, '{loading}')",
                                        loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                    ))
                                {
                                    span { "Make Admin" }
                                }
                            }
                            button
                                class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM)
                                data-init=(format!("this.triggerButton = getPopoverTrigger(this), this.fetch = fetch('{}')", bitsync_routes::GetResetUserTotpDialog { user_id: user.id }))
                                data-on-click="closeClosestPopover(this), this.fetch.trigger()"
                                data-effect=(format!(
                                    "handleButtonLoading(this.triggerButton, this.fetch, '{loading}')",
                                    loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                ))
                            {
                                span { "Reset TOTP" }
                            }

                            div class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_DIVIDER) {}

                            @if user.is_suspended {
                                button
                                    class=(crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM)
                                    data-init=(format!("this.triggerButton = getPopoverTrigger(this), this.fetch = fetch('{}')", bitsync_routes::GetUnsuspendUserDialog { user_id: user.id }))
                                    data-on-click="closeClosestPopover(this), this.fetch.trigger()"
                                    data-effect=(format!(
                                        "handleButtonLoading(this.triggerButton, this.fetch, '{loading}')",
                                        loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                    ))
                                {
                                    span { "Unsuspend User" }
                                }
                            } @else {
                                button
                                    class=(
                                        crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM, " ",
                                        crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM_DANGER,
                                    )
                                    data-init=(format!("this.triggerButton = getPopoverTrigger(this), this.fetch = fetch('{}')", bitsync_routes::GetSuspendUserDialog { user_id: user.id }))
                                    data-on-click="closeClosestPopover(this), this.fetch.trigger()"
                                    data-effect=(format!(
                                        "handleButtonLoading(this.triggerButton, this.fetch, '{loading}')",
                                        loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                    ))
                                {
                                    span { "Suspend User" }
                                }
                            }
                            button
                                class=(
                                    crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM, " ",
                                    crate::styles::context_menu::ClassName::CONTEXT_MENU_ITEM_DANGER,
                                )
                                data-init=(format!("this.triggerButton = getPopoverTrigger(this), this.fetch = fetch('{}')", bitsync_routes::GetDeleteUserDialog { user_id: user.id }))
                                data-on-click="closeClosestPopover(this), this.fetch.trigger()"
                                data-effect=(format!(
                                    "handleButtonLoading(this.triggerButton, this.fetch, '{loading}')",
                                    loading = crate::styles::button::ClassName::BUTTON_LOADING,
                                ))
                            {
                                span { "Delete User" }
                            }
                        }
                    }
                }
            }
        }
        .render_to(buffer);
    }
}
