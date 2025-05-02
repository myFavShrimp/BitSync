use maud::Render;

pub enum LoginPage {
    Login(LoginForm),
    Totp(TotpForm),
}

impl Default for LoginPage {
    fn default() -> Self {
        Self::Login(Default::default())
    }
}

impl Render for LoginPage {
    fn render(&self) -> maud::Markup {
        super::base::GuestDocument(maud::html! {
            main class=(crate::styles::login_page::ClassName::PAGE) {
                style { (crate::styles::login_page::STYLE_SHEET) }

                (crate::icons::logo::Logo::with_class(crate::styles::login_page::ClassName::LOGO))

                @match &self {
                    LoginPage::Login(login_form) => (login_form),
                    LoginPage::Totp(totp_form) => (totp_form),
                }
            }
        })
        .render()
    }
}

#[derive(Default)]
pub struct LoginForm {
    pub username: Option<String>,
    pub error_message: Option<String>,
}

impl Render for LoginForm {
    fn render(&self) -> maud::Markup {
        maud::html! {
            form class=(crate::styles::login_page::ClassName::FORM) hx-post=(bitsync_routes::PostLoginAction.to_string()) hx-target="this" {
                div {
                    div {
                        label {
                            "Username"
                        }
                        input value=[&self.username] name="username" required;
                    }
                    div {
                        label {
                            "Password"
                        }
                        input type="password" required name="password";
                    }
                    @match &self.error_message {
                        Some(message) => {
                            div {
                                (message)
                            }
                        }
                        None => {}
                    }
                    div {
                        a href=(bitsync_routes::GetRegisterPage.to_string()) {
                            "I don't have an account"
                        }
                        button type="submit" {
                            "Login"
                        }
                    }
                }
            }
        }
    }
}

pub struct TotpForm {
    pub error_message: Option<String>,
}

impl Render for TotpForm {
    fn render(&self) -> maud::Markup {
        maud::html! {
            form class=(crate::styles::login_page::ClassName::FORM) hx-post=(bitsync_routes::PostLoginTotpAuthAction.to_string()) hx-target="this" {
                div {
                    div {
                        label {
                            "TOTP Code"
                        }
                        input name="totp" required;
                    }
                    @match &self.error_message {
                        Some(message) => {
                            div {
                                (message)
                            }
                        }
                        None => {}
                    }
                    button type="submit" {
                        "Login"
                    }
                }
            }
        }
    }
}
