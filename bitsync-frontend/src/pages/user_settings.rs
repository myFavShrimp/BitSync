use bitsync_database::entity::User;

use hypertext::prelude::*;

use crate::pages::base::LoggedInDocument;

pub struct UserSettingsPage {
    pub user: User,
}

impl From<User> for UserSettingsPage {
    fn from(value: User) -> Self {
        Self { user: value }
    }
}

impl Renderable for UserSettingsPage {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            LoggedInDocument {
                style {
                    (crate::styles::files_home_page::STYLE_SHEET)
                }
                main {
                    h1 {
                        "Account"
                    }
                    ChangePasswordForm;
                }
            }
        }
        .render_to(buffer);
    }
}

struct ChangePasswordForm;

impl Renderable for ChangePasswordForm {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            form /*hx-post=(bitsync_routes::PostUserSettingsChangePassword.to_string()) hx-target="this"*/ {
                label {
                    "current password"
                    input type="password" name="current_password";
                }
                label {
                    "new password"
                    input type="password" name="new_password";
                }
                label {
                    "repeat new password"
                    input type="password" name="new_password_repeated";
                }
                button {
                    "Save"
                }
                button type="reset" {
                    "Cancel"
                }
            }
        }
        .render_to(buffer);
    }
}
