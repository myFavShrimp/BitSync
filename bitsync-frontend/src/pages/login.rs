use maud::Render;

#[derive(Debug, Default)]
pub struct LoginPage {
    pub username: Option<String>,
    pub error_message: Option<String>,
}

impl Render for LoginPage {
    fn render(&self) -> maud::Markup {
        super::base::GuestDocument(maud::html! {
            main class=(crate::styles::login_page::ClassName::PAGE) {
                style { (crate::styles::login_page::STYLE_SHEET) }

                (crate::icons::logo::Logo::with_class(crate::styles::login_page::ClassName::LOGO))

                (LoginForm { username: self.username.clone(), error_message: self.error_message.clone() })
            }
        })
        .render()
    }
}

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
