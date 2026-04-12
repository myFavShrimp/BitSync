use hypertext::prelude::*;

use crate::{Component, error_banner::OptionalErrorBanner, totp::totp_qr_src};

#[derive(Clone)]
pub enum TotpDisplayError {
    InvalidCode,
    InternalServerError,
}

impl TotpDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidCode => "The entered TOTP code is invalid",
            Self::InternalServerError => "An internal server error occurred",
        }
    }
}

static TOTP_TAB_CONTENT_ID: &str = "totp-tab-content";

#[derive(Clone)]
pub enum TotpTabContent {
    Prompt,
    Setup {
        totp_secret_image_base64_img_src: String,
        totp_secret: String,
        error: Option<TotpDisplayError>,
    },
    RecoveryCodes {
        recovery_codes: Vec<String>,
    },
}

impl Component for TotpTabContent {
    fn id(&self) -> String {
        TOTP_TAB_CONTENT_ID.to_owned()
    }
}

impl Renderable for TotpTabContent {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            @match self {
                TotpTabContent::Prompt => {
                    form
                        id=(self.id())
                        class=(crate::styles::modal::ClassName::MODAL_BODY)
                        data-hijack
                        action=(bitsync_routes::PostUserSettingsTotpInitiateReset.to_string())
                        method="POST"
                    {
                        p class=(crate::styles::modal::ClassName::MODAL_DESCRIPTION) {
                            "Two-factor authentication is active on your account. Resetting will generate a new TOTP secret and new recovery codes. Your old authenticator entry and recovery codes will stop working when you confirm the new TOTP code."
                        }

                        button
                            type="submit"
                            class=(
                                crate::styles::button::ClassName::BUTTON, " ",
                                crate::styles::button::ClassName::BUTTON_DANGER,
                            )
                        {
                            "Reset Two-Factor Authentication"
                        }
                    }
                }
                TotpTabContent::Setup { totp_secret_image_base64_img_src, totp_secret, error } => {
                    form
                        id=(self.id())
                        class=(crate::styles::modal::ClassName::MODAL_BODY)
                        data-hijack
                        action=(bitsync_routes::PostUserSettingsTotpReset.to_string())
                        method="POST"
                    {
                        p class=(crate::styles::modal::ClassName::MODAL_DESCRIPTION) {
                            "Scan the QR code with your authenticator app (Google Authenticator, Authy, etc.), then enter the generated code below to confirm. Your old TOTP remains active until you confirm the new code."
                        }

                        img
                            class=(crate::styles::user_settings_page::ClassName::TOTP_QR_CODE)
                            src=(totp_qr_src(totp_secret_image_base64_img_src));

                        details class=(crate::styles::user_settings_page::ClassName::TOTP_MANUAL_SECRET) {
                            summary {
                                "Can't scan? Show manual entry code"
                            }
                            pre { code { (totp_secret) } }
                        }

                        div class=(crate::styles::modal::ClassName::FORM_DIVIDER) {}

                        label class=(crate::styles::modal::ClassName::FORM_LABEL) {
                            "TOTP Code"

                            input
                                class=(crate::styles::base::ClassName::FORM_CONTROL)
                                type="text"
                                name="totp"
                                placeholder="Enter the 6-digit code"
                                required;
                        }

                        OptionalErrorBanner message=(error.as_ref().map(|error| error.message().to_owned()));

                        button
                            type="submit"
                            class=(
                                crate::styles::button::ClassName::BUTTON, " ",
                                crate::styles::button::ClassName::BUTTON_PRIMARY,
                            )
                        {
                            "Confirm"
                        }
                    }
                }
                TotpTabContent::RecoveryCodes { recovery_codes } => {
                    div
                        id=(self.id())
                        class=(crate::styles::modal::ClassName::MODAL_BODY)
                    {
                        p class=(crate::styles::modal::ClassName::MODAL_DESCRIPTION) {
                            ("To ensure you don't lose access to your account, please save these recovery codes in a secure location.")
                        }

                        p class=(crate::styles::modal::ClassName::MODAL_DESCRIPTION) {
                            ("If you ever lose access to your authenticator app, you can enter any of these codes in the TOTP field when signing in. Each code works only once.")
                        }

                        p class=(crate::styles::modal::ClassName::MODAL_DESCRIPTION) {
                            ("These codes will only be shown now. If you navigate away without saving them, you'll need to generate new ones.")
                        }

                        div class=(crate::styles::modal::ClassName::FORM_DIVIDER) {}

                        div class=(crate::styles::user_settings_page::ClassName::RECOVERY_CODES_GRID) {
                            @for recovery_code in recovery_codes {
                                div class=(crate::styles::user_settings_page::ClassName::RECOVERY_CODE) {
                                    (recovery_code)
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
