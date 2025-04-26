use bitsync_database::entity::User;

use maud::Render;

pub struct UserSettingsPage {
    pub user: User,
}

impl From<User> for UserSettingsPage {
    fn from(value: User) -> Self {
        Self { user: value }
    }
}

impl Render for UserSettingsPage {
    fn render(&self) -> maud::Markup {
        super::base::LoggedInDocument(maud::html! {
            style {
                (crate::styles::files_home_page::STYLE_SHEET)
            }
            main {
                h1 {
                    "Account"
                }
                (ChangePasswordForm)
            }
        })
        .render()
    }
}

struct ChangePasswordForm;

impl Render for ChangePasswordForm {
    fn render(&self) -> maud::Markup {
        maud::html! {
            form hx-post=(bitsync_routes::PostUserSettingsChangePassword.to_string()) hx-target="this" {
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
    }
}
