use bitsync_core::use_case::auth::setup_totp::TotpSetupResult;
use maud::Render;

use crate::{error_banner::optional_error_banner, totp::totp_qr_src};

pub enum RegisterPage {
    UserRegistration(RegisterForm),
    TotpSetup(TotpSetupForm),
}

impl Default for RegisterPage {
    fn default() -> Self {
        Self::UserRegistration(Default::default())
    }
}

impl Render for RegisterPage {
    fn render(&self) -> maud::Markup {
        super::base::GuestDocument(maud::html! {
            style { (maud::PreEscaped(crate::styles::register_page::STYLE_SHEET)) }

            (crate::icons::logo::Logo::with_class(crate::styles::register_page::ClassName::LOGO))
            p class=(crate::styles::register_page::ClassName::PAGE_HINT) {("Create an account to get started")}

            main {
                @match &self {
                    Self::UserRegistration(register_form) => (register_form),
                    Self::TotpSetup(totp_setup_form) => (totp_setup_form),
                }
            }
        })
        .render()
    }
}

#[derive(Default)]
pub struct RegisterForm {
    pub username: Option<String>,
    pub error_message: Option<String>,
}

static PAGE_FORM_SWAP_ID: &str = "register-page-form";

impl Render for RegisterForm {
    fn render(&self) -> maud::Markup {
        maud::html! {
            form class=(crate::styles::register_page::ClassName::FORM) hx-swap-oob=(format!("outerHTML:#{PAGE_FORM_SWAP_ID}")) hx-post=(bitsync_routes::PostRegisterAction.to_string()) id=(PAGE_FORM_SWAP_ID) {
                (optional_error_banner(&self.error_message))

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
    }
}

#[derive(Default)]
pub struct TotpSetupForm {
    pub totp_secret_image_base64_img_src: String,
    pub totp_secret: String,
    pub error_message: Option<String>,
}

impl Render for TotpSetupForm {
    fn render(&self) -> maud::Markup {
        maud::html! {
            form class=(crate::styles::register_page::ClassName::FORM) hx-post=(bitsync_routes::PostRegisterTotpSetupAction.to_string()) hx-target="this" id=(PAGE_FORM_SWAP_ID) hx-swap-oob=(format!("outerHTML:#{PAGE_FORM_SWAP_ID}")) {
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

                (optional_error_banner(&self.error_message))

                label class=(crate::styles::register_page::ClassName::INPUT_WRAPPER) {
                    "TOTP Code"

                    div class=(crate::styles::register_page::ClassName::TOTP_INPUT_WRAPPER) {
                        input class=(crate::styles::base::ClassName::FORM_CONTROL) name="totp" placeholder="Enter your one-time password" required;

                        p id="totp-timer" class=(crate::styles::register_page::ClassName::TOTP_TIMER) {"30"}

                        script {(maud::PreEscaped(r#"
                            setInterval(() => {
                                const totpTimer = document.querySelector('#totp-timer');
                                const time = 30 - (Math.floor(Date.now() / 1000) % 30);
                                totpTimer.textContent = time;
                                totpTimer.style.background = `conic-gradient(var(--timer-pie-color) ${time/30*360}deg, rgba(255, 255, 255, 0.1) 0deg)`;
                            }, 100);
                        "#))}
                    }
                }
                button hx-post type="submit" class=(crate::styles::base::ClassName::BUTTON) {
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
            div class=(crate::styles::register_page::ClassName::FORM) hx-swap-oob=(format!("outerHTML:#{PAGE_FORM_SWAP_ID}")) id=(PAGE_FORM_SWAP_ID) {
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
    }
}
