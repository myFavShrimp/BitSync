use bitsync_core::use_case::auth::retrieve_totp_setup_data::TotpSetupData;
use bitsync_core::use_case::auth::setup_totp::TotpSetupResult;
use maud::Render;

use crate::totp::totp_qr_src;

pub struct TotpSetupPage {
    pub totp_secret_image_base64_img_src: String,
    pub totp_secret: String,
}

impl From<TotpSetupData> for TotpSetupPage {
    fn from(value: TotpSetupData) -> Self {
        Self {
            totp_secret_image_base64_img_src: totp_qr_src(value.secret_base64_qr_code),
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
            totp_secret_image_base64_img_src: totp_qr_src(value.secret_base64_qr_code),
            totp_secret: value.secret_base32,
        }
    }
}

impl Render for TotpSetupForm {
    fn render(&self) -> maud::Markup {
        maud::html! {
            form hx-target="this" id="blabla" {
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

pub struct TotpRecoveryCodesPrompt {
    recovery_codes: Vec<String>,
}

impl From<TotpSetupResult> for TotpRecoveryCodesPrompt {
    fn from(value: TotpSetupResult) -> Self {
        Self {
            recovery_codes: value.recovery_codes,
        }
    }
}

impl Render for TotpRecoveryCodesPrompt {
    fn render(&self) -> maud::Markup {
        maud::html! {
            template {
                div hx-swap-oob="outerHTML:#blabla" {
                    p {
                        "Save the recovery codes:"
                    }
                    ul {
                        @for recovery_code in &self.recovery_codes {
                            li {
                                code { "`" (recovery_code) "`" }
                            }
                        }
                    }
                    a href=(bitsync_routes::GetFilesHomePage.to_string()) {
                        "I saved the codes"
                    }
                }
            }
        }
    }
}
