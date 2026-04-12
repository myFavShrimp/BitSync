use bitsync_database::entity::User;
use hypertext::prelude::*;

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

                div class=(crate::styles::user_settings_page::ClassName::USER_LIST) {
                    @for user in &self.users {
                        @let popover_id = format!("user-actions-{}", user.id);

                        div class=(crate::styles::user_settings_page::ClassName::USER_ITEM) {
                            div class=(crate::styles::user_settings_page::ClassName::USER_INFO) {
                                span class=(crate::styles::user_settings_page::ClassName::USER_NAME) {
                                    (user.username)
                                }
                                @if user.is_admin {
                                    span class=(crate::styles::user_settings_page::ClassName::USER_ADMIN_BADGE) {
                                        "Admin"
                                    }
                                }
                            }

                            button
                                class=(crate::styles::user_settings_page::ClassName::USER_SETTINGS_BUTTON)
                                popovertarget=(popover_id)
                                title="User settings"
                            {
                                (crate::icons::ellipsis_vertical::EllipsisVertical)
                            }

                            div
                                id=(popover_id)
                                class=(
                                    crate::styles::base::ClassName::CONTEXT_MENU, " ",
                                    crate::styles::user_settings_page::ClassName::USER_CONTEXT_MENU,
                                )
                                popover
                            {
                                button
                                    class=(
                                        crate::styles::base::ClassName::CONTEXT_MENU_ITEM, " ",
                                        crate::styles::modal::ClassName::DISABLED,
                                    )
                                    disabled
                                {
                                    span { "Make Admin" }
                                }
                                button
                                    class=(
                                        crate::styles::base::ClassName::CONTEXT_MENU_ITEM, " ",
                                        crate::styles::modal::ClassName::DISABLED,
                                    )
                                    disabled
                                {
                                    span { "Reset TOTP" }
                                }

                                div class=(crate::styles::base::ClassName::CONTEXT_MENU_DIVIDER) {}

                                button
                                    class=(
                                        crate::styles::base::ClassName::CONTEXT_MENU_ITEM, " ",
                                        crate::styles::base::ClassName::CONTEXT_MENU_ITEM_DANGER, " ",
                                        crate::styles::modal::ClassName::DISABLED,
                                    )
                                    disabled
                                {
                                    span { "Suspend User" }
                                }
                                button
                                    class=(
                                        crate::styles::base::ClassName::CONTEXT_MENU_ITEM, " ",
                                        crate::styles::base::ClassName::CONTEXT_MENU_ITEM_DANGER, " ",
                                        crate::styles::modal::ClassName::DISABLED,
                                    )
                                    disabled
                                {
                                    span { "Delete User" }
                                }
                            }
                        }
                    }
                }
            }
        }
        .render_to(buffer);
    }
}
