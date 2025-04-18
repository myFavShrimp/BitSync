use bitsync_core::use_case::user_settings::retrieve_totp_setup_data::TotpSetupData;
use bitsync_database::entity::User;

use maud::Render;

static BASE64_PNG_IMG_SRC_PREFIX: &str = "data:image/png;base64";

pub struct TotpSetupPage {
    pub totp_secret_image_base64_img_src: String,
    pub totp_secret: String,
}

impl From<TotpSetupData> for TotpSetupPage {
    fn from(value: TotpSetupData) -> Self {
        Self {
            totp_secret_image_base64_img_src: format!(
                "{},{}",
                BASE64_PNG_IMG_SRC_PREFIX, value.secret_base64_qr_code
            ),
            totp_secret: value.secret_base32,
        }
    }
}

impl Render for TotpSetupPage {
    fn render(&self) -> maud::Markup {
        super::base::LoggedInDocument(maud::html! {
            style { (crate::styles::files_home_page::STYLE_SHEET) }

            main {
                h1 { "TOTP Setup" }

                (TotpSetupForm { totp_secret_image_base64_img_src: self.totp_secret_image_base64_img_src.clone(), totp_secret: self.totp_secret.clone() })
            }
        }).render()
    }
}

pub struct TotpSetupForm {
    pub totp_secret_image_base64_img_src: String,
    pub totp_secret: String,
}

impl From<TotpSetupData> for TotpSetupForm {
    fn from(value: TotpSetupData) -> Self {
        Self {
            totp_secret_image_base64_img_src: format!(
                "{},{}",
                BASE64_PNG_IMG_SRC_PREFIX, value.secret_base64_qr_code
            ),
            totp_secret: value.secret_base32,
        }
    }
}

impl Render for TotpSetupForm {
    fn render(&self) -> maud::Markup {
        maud::html! {
            form hx-target {
                img src=(self.totp_secret_image_base64_img_src);
                p {
                    "Scan the QR code to add the totp code to your authenticator app."
                }
                details {
                    summary {
                        "or add the totp manually"
                    }
                    "Insert the following secret into your authenticator:"
                    code { "`" (self.totp_secret) "`" }
                }
                input name="totp" required;
                button hx-post {
                    "send"
                }
            }
        }
    }
}

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
