use bitsync_database::entity::User;

use crate::handler::routes::PostUserSettingsChangePassword;

#[derive(askama::Template)]
#[template(path = "user_settings_page.html")]
pub struct UserSettingsPage {
    pub user: User,
    pub change_password_url: String,
}

impl From<User> for UserSettingsPage {
    fn from(value: User) -> Self {
        let change_password_url = PostUserSettingsChangePassword.to_string();

        Self {
            user: value,
            change_password_url,
        }
    }
}
