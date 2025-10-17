use bitsync_core::use_case::auth::setup_totp::TotpSetupResult;
use hypertext::{Raw, prelude::*};

use crate::{
    Component, error_banner::OptionalErrorBanner, pages::base::GuestDocument, totp::totp_qr_src,
};

pub enum RegisterPage {
    UserRegistration(RegisterForm),
    TotpSetup(TotpSetupForm),
}

impl Default for RegisterPage {
    fn default() -> Self {
        Self::UserRegistration(Default::default())
    }
}

impl Renderable for RegisterPage {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            GuestDocument {
                style { (Raw::dangerously_create(crate::styles::register_page::STYLE_SHEET)) }

                // (crate::icons::logo::Logo::with_class(crate::styles::register_page::ClassName::LOGO)) TODO
                p class=(crate::styles::register_page::ClassName::PAGE_HINT) {("Create an account to get started")}

                main {
                    @match &self {
                        Self::UserRegistration(register_form) => (register_form),
                        Self::TotpSetup(totp_setup_form) => (totp_setup_form),
                    }
                }
            }
        }
        .render_to(buffer);
    }
}

#[derive(Default)]
pub struct RegisterForm {
    pub username: Option<String>,
    pub error_message: Option<String>,
}

impl Component for RegisterForm {
    fn id(&self) -> String {
        "register-form".to_owned()
    }
}

impl Renderable for RegisterForm {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            form
                id=(self.id())
                class=(crate::styles::register_page::ClassName::FORM)
                data-hijack
                action=(bitsync_routes::PostRegisterAction.to_string())
                method="POST"
            {
                OptionalErrorBanner message=(self.error_message.clone());

                label class=(crate::styles::register_page::ClassName::INPUT_WRAPPER) {
                    "Username"
                    div class=(crate::styles::register_page::ClassName::INPUT) {
                        input class=(crate::styles::base::ClassName::FORM_CONTROL) value=[&self.username] name="username" placeholder="Enter your username" required;
                    }
                }
                label class=(crate::styles::register_page::ClassName::INPUT_WRAPPER) {
                    "Password"
                    div class=(crate::styles::register_page::ClassName::INPUT) {
                        input class=(crate::styles::base::ClassName::FORM_CONTROL) type="password" placeholder="Enter your password" required name="password";
                    }
                }
                div class=(crate::styles::register_page::ClassName::ACTIONS) {
                    button type="submit" class=(crate::styles::base::ClassName::BUTTON) {
                        "Register"
                    }
                    a href=(bitsync_routes::GetLoginPage.to_string()) class=(crate::styles::base::ClassName::TEXT_LINK) {
                        "I already have an account"
                    }
                }
            }
        }
        .render_to(buffer);
    }
}

static REGISTER_TOTP_FORM_ID: &str = "register-totp-form";

#[derive(Default)]
pub struct TotpSetupForm {
    pub totp_secret_image_base64_img_src: String,
    pub totp_secret: String,
    pub error_message: Option<String>,
}

impl Component for TotpSetupForm {
    fn id(&self) -> String {
        REGISTER_TOTP_FORM_ID.to_owned()
    }
}

impl Renderable for TotpSetupForm {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            form
                id=(self.id())
                class=(crate::styles::register_page::ClassName::FORM)
                data-hijack
                action=(bitsync_routes::PostRegisterTotpSetupAction.to_string())
                method="POST"
            {
                div class=(crate::styles::register_page::ClassName::TOTP_HEADER) {
                    h1 {"Two-Factor Authentication Setup"}
                    p {"Scan the QR code with your authenticator app (Google Authenticator, Authy, etc.)"}
                }
                div class=(crate::styles::register_page::ClassName::TOTP_QR_WRAPPER) {
                    img src=(totp_qr_src(&self.totp_secret_image_base64_img_src));
                }
                details class=(crate::styles::register_page::ClassName::TOTP_SECRET) {
                    summary {
                        "Can't scan? Show manual entry code"
                    }
                    pre { code { (self.totp_secret) } }
                }

                hr;

                OptionalErrorBanner message=(self.error_message.clone());

                label class=(crate::styles::register_page::ClassName::INPUT_WRAPPER) {
                    "TOTP Code"

                    div class=(crate::styles::register_page::ClassName::TOTP_INPUT_WRAPPER) {
                        input class=(crate::styles::base::ClassName::FORM_CONTROL) name="totp" placeholder="Enter your one-time password" required;

                        p id="totp-timer" class=(crate::styles::register_page::ClassName::TOTP_TIMER) {"30"}

                        script {(Raw::dangerously_create(r#"
                            setInterval(() => {
                                const totpTimer = document.querySelector('#totp-timer');
                                const time = 30 - (Math.floor(Date.now() / 1000) % 30);
                                totpTimer.textContent = time;
                                totpTimer.style.background = `conic-gradient(var(--timer-pie-color) ${time/30*360}deg, rgba(255, 255, 255, 0.1) 0deg)`;
                            }, 100);
                        "#))}
                    }
                }
                button type="submit" class=(crate::styles::base::ClassName::BUTTON) {
                    "send"
                }
            }
        }
        .render_to(buffer);
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

impl Component for TotpRecoveryCodesPrompt {
    fn id(&self) -> String {
        REGISTER_TOTP_FORM_ID.to_owned()
    }
}

impl Renderable for TotpRecoveryCodesPrompt {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            div
                id=(self.id())
                class=(crate::styles::register_page::ClassName::FORM)
            {
                div class=(crate::styles::register_page::ClassName::TOTP_HEADER) {
                    h1 {"Save Your Recovery Codes"}
                    p {"Your two-factor authentication is now active."}
                }

                p {"To ensure you don't lose access to your account, please save these recovery codes in a secure location."}
                p {"These codes will only be shown once. If you navigate away without saving them, you'll need to generate new codes."}

                details class=(crate::styles::register_page::ClassName::TOTP_SECRET) open {
                    summary {
                        "Recovery Codes"
                    }

                    div class=(crate::styles::register_page::ClassName::RECOVERY_CODES) {
                        @for recovery_code in &self.recovery_codes {
                            pre {
                                code {
                                    (recovery_code)
                                }
                            }
                        }
                    }
                }
                a class=(crate::styles::base::ClassName::BUTTON) href=(bitsync_routes::GetFilesHomePage.to_string()) {
                    "Continue"
                }
            }
        }
        .render_to(buffer);
    }
}
