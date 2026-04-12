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
                        div class=(crate::styles::user_settings_page::ClassName::USER_ITEM) {
                            span class=(crate::styles::user_settings_page::ClassName::USER_NAME) {
                                (user.username)
                            }
                            @if user.is_admin {
                                span class=(crate::styles::user_settings_page::ClassName::USER_ADMIN_BADGE) {
                                    "Admin"
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
