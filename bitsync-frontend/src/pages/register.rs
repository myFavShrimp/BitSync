use bitsync_core::use_case::auth::setup_totp::TotpSetupResult;
use maud::Render;

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
            main class=(crate::styles::register_page::ClassName::PAGE) {
                style { (crate::styles::register_page::STYLE_SHEET) }

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
            form class=(crate::styles::register_page::ClassName::FORM) hx-post=(bitsync_routes::PostRegisterAction.to_string()) id=(PAGE_FORM_SWAP_ID) {
                (crate::icons::logo::Logo::with_class(crate::styles::register_page::ClassName::LOGO))
                div {
                    div {
                        label {
                            "Username"
                        }
                        input name="username" required;
                    }
                    div {
                        label {
                            "Password"
                        }
                        input required type="password" name="password";
                    }
                    div {
                        a."button border large" href=(bitsync_routes::GetLoginPage.to_string()) {
                            "I already have an account"
                        }
                        button type="submit" {
                            "Register"
                        }
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
            form hx-target="this" id=(PAGE_FORM_SWAP_ID) {
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
                div hx-swap-oob=(format!("outerHTML:#{PAGE_FORM_SWAP_ID}")) id=(PAGE_FORM_SWAP_ID) {
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
