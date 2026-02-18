use hypertext::prelude::*;

use crate::{Component, pages::base::LoggedInDocument};

pub struct UserSettingsPage;

impl Renderable for UserSettingsPage {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            LoggedInDocument {
                style { (crate::styles::user_settings_page::STYLE_SHEET) }
                main {
                    h1 {
                        "Account Settings"
                    }
                    ChangePasswordForm;
                }
            }
        }
        .render_to(buffer);
    }
}

struct ChangePasswordForm;

impl Component for ChangePasswordForm {
    fn id(&self) -> String {
        "change-password-form".to_owned()
    }
}

impl Renderable for ChangePasswordForm {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            form
                id=(self.id())
                data-hijack
                action=(bitsync_routes::PostUserSettingsChangePassword.to_string())
                method="POST"
            {
                label {
                    "Current Password"
                    input type="password" name="current_password";
                }
                label {
                    "New Password"
                    input type="password" name="new_password";
                }
                label {
                    "Repeat New Password"
                    input type="password" name="new_password_repeated";
                }
                button type="submit" {
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
