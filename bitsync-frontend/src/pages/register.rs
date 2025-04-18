use maud::Render;

#[derive(Default)]
pub struct RegisterPage {
    pub username: Option<String>,
    pub error_message: Option<String>,
}

impl Render for RegisterPage {
    fn render(&self) -> maud::Markup {
        super::base::GuestDocument(maud::html! {
            main class=(crate::styles::register_page::ClassName::PAGE) {
                style { (crate::styles::register_page::STYLE_SHEET) }
                form class=(crate::styles::register_page::ClassName::FORM) hx-post=(bitsync_routes::PostRegisterAction.to_string()) {
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
        }).render()
    }
}
