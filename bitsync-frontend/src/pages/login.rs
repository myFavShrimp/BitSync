use hypertext::prelude::*;

use crate::{Component, error_banner::OptionalErrorBanner, pages::base::GuestDocument};

pub enum LoginDisplayError {
    InvalidCredentials,
    InternalServerError,
}

impl LoginDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidCredentials => "Invalid username or password",
            Self::InternalServerError => "An internal server error occurred",
        }
    }
}

pub enum TotpVerificationDisplayError {
    InvalidCode,
    NotSetUp,
    InternalServerError,
}

impl TotpVerificationDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidCode => "Invalid verification code",
            Self::NotSetUp => "TOTP is not set up",
            Self::InternalServerError => "An internal server error occurred",
        }
    }
}

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
                style { (crate::styles::login_page::STYLE_SHEET) }

                (crate::icons::logo::Logo::with_class(crate::styles::login_page::ClassName::LOGO))

                p
                    class=(crate::styles::login_page::ClassName::PAGE_HINT)
                {
                    ("Sign in to access your secure storage")
                }

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
    pub error: Option<LoginDisplayError>,
}

impl Component for LoginForm {
    fn id(&self) -> String {
        "login-form".to_owned()
    }
}

impl Renderable for LoginForm {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            form
                id=(self.id())
                class=(crate::styles::login_page::ClassName::FORM)
                data-hijack
                action=(bitsync_routes::PostLoginAction.to_string())
                method="POST"
            {
                label class=(crate::styles::login_page::ClassName::INPUT_WRAPPER) {
                    "Username"

                    div class=(crate::styles::login_page::ClassName::INPUT) {
                        input
                            class=(crate::styles::base::ClassName::FORM_CONTROL)
                            value=[&self.username]
                            name="username"
                            placeholder="Enter your username"
                            required;
                    }
                }
                label class=(crate::styles::login_page::ClassName::INPUT_WRAPPER) {
                    "Password"

                    div class=(crate::styles::login_page::ClassName::INPUT) {
                        input
                            class=(crate::styles::base::ClassName::FORM_CONTROL)
                            type="password"
                            placeholder="Enter your password"
                            required
                            name="password";
                    }
                }

                OptionalErrorBanner message=(self.error.as_ref().map(|error| error.message().to_owned()));

                div class=(crate::styles::login_page::ClassName::ACTIONS) {
                    button
                        type="submit"
                        class=(crate::styles::base::ClassName::BUTTON)
                    {
                        "Sign in"
                    }

                    a
                        href=(bitsync_routes::GetRegisterPage.to_string())
                        class=(crate::styles::base::ClassName::TEXT_LINK)
                    {
                        "I don't have an account"
                    }
                }
            }
        }.render_to(buffer);
    }
}

pub struct TotpForm {
    pub error: Option<TotpVerificationDisplayError>,
}

impl Component for TotpForm {
    fn id(&self) -> String {
        "login-totp-form".to_owned()
    }
}

impl Renderable for TotpForm {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            form
                id=(self.id())
                class=(crate::styles::login_page::ClassName::FORM)
                data-hijack
                action=(bitsync_routes::PostLoginTotpAuthAction.to_string())
                method="POST"
            {
                label class=(crate::styles::login_page::ClassName::INPUT_WRAPPER) {
                    "TOTP Code"

                    div class=(crate::styles::login_page::ClassName::TOTP_INPUT_WRAPPER) {
                        input
                            class=(crate::styles::base::ClassName::FORM_CONTROL)
                            name="totp"
                            placeholder="Enter your one-time password"
                            required;

                        p
                            id="totp-timer"
                            class=(crate::styles::login_page::ClassName::TOTP_TIMER)
                        {
                            "30"
                        }

                        script {(hypertext::Raw::dangerously_create(r#"
                            setInterval(() => {
                                const totpTimer = document.querySelector('#totp-timer');
                                const time = 30 - (Math.floor(Date.now() / 1000) % 30);
                                totpTimer.textContent = time;
                                totpTimer.style.background = `conic-gradient(var(--timer-pie-color) ${time/30*360}deg, rgba(255, 255, 255, 0.1) 0deg)`;
                            }, 100);
                        "#))}
                    }
                }

                OptionalErrorBanner message=(self.error.as_ref().map(|error| error.message().to_owned()));

                button
                    type="submit"
                    class=(crate::styles::base::ClassName::BUTTON)
                {
                    "Login"
                }
            }
        }.render_to(buffer);
    }
}
