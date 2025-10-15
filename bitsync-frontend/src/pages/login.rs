use hypertext::{Raw, prelude::*};

use crate::{error_banner::OptionalErrorBanner, pages::base::GuestDocument};

pub enum LoginPage {
    Login(LoginForm),
    Totp(TotpForm),
}

impl Default for LoginPage {
    fn default() -> Self {
        Self::Login(Default::default())
    }
}

impl Renderable for LoginPage {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            GuestDocument {
                style { (Raw::dangerously_create(crate::styles::login_page::STYLE_SHEET)) }

                // (crate::icons::logo::Logo::with_class(crate::styles::login_page::ClassName::LOGO)) TODO
                p class=(crate::styles::login_page::ClassName::PAGE_HINT) {("Sign in to access your secure storage")}

                main {
                    @match &self {
                        LoginPage::Login(login_form) => (login_form),
                        LoginPage::Totp(totp_form) => (totp_form),
                    }
                }
            }
        }
        .render_to(buffer);
    }
}

#[derive(Default)]
pub struct LoginForm {
    pub username: Option<String>,
    pub error_message: Option<String>,
}

impl Renderable for LoginForm {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            form class=(crate::styles::login_page::ClassName::FORM) /*hx-post=(bitsync_routes::PostLoginAction.to_string()) hx-target="this" TODO */ {
                OptionalErrorBanner message=(self.error_message.clone());

                label class=(crate::styles::login_page::ClassName::INPUT_WRAPPER) {
                    "Username"
                    div class=(crate::styles::login_page::ClassName::INPUT) {
                        input class=(crate::styles::base::ClassName::FORM_CONTROL) value=[&self.username] name="username" placeholder="Enter your username" required;
                    }
                }
                label class=(crate::styles::login_page::ClassName::INPUT_WRAPPER) {
                    "Password"
                    div class=(crate::styles::login_page::ClassName::INPUT) {
                        input class=(crate::styles::base::ClassName::FORM_CONTROL) type="password" placeholder="Enter your password" required name="password";
                    }
                }
                @match &self.error_message {
                    Some(message) => {
                        div {
                            (message)
                        }
                    }
                    None => {}
                }
                div class=(crate::styles::login_page::ClassName::ACTIONS) {
                    button type="submit" class=(crate::styles::base::ClassName::BUTTON) {
                        "Sign in"
                    }
                    a href=(bitsync_routes::GetRegisterPage.to_string()) class=(crate::styles::base::ClassName::TEXT_LINK) {
                        "I don't have an account"
                    }
                }
            }
        }.render_to(buffer);
    }
}

pub struct TotpForm {
    pub error_message: Option<String>,
}

impl Renderable for TotpForm {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            form class=(crate::styles::login_page::ClassName::FORM) /*hx-post=(bitsync_routes::PostLoginTotpAuthAction.to_string()) hx-target="this" TODO*/ {
                OptionalErrorBanner message=(self.error_message.clone());

                label class=(crate::styles::login_page::ClassName::INPUT_WRAPPER) {
                    "TOTP Code"

                    div class=(crate::styles::login_page::ClassName::TOTP_INPUT_WRAPPER) {
                        input class=(crate::styles::base::ClassName::FORM_CONTROL) name="totp" placeholder="Enter your one-time password" required;

                        p id="totp-timer" class=(crate::styles::login_page::ClassName::TOTP_TIMER) {"30"}

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
                @match &self.error_message {
                    Some(message) => {
                        div {
                            (message)
                        }
                    }
                    None => {}
                }
                button type="submit" class=(crate::styles::base::ClassName::BUTTON) {
                    "Login"
                }
            }
        }.render_to(buffer);
    }
}
