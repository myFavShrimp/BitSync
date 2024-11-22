#[derive(askama::Template, Default)]
#[template(path = "login_page.html")]
pub struct LoginPage {
    pub username: String,
    pub error_message: Option<String>,
}

#[derive(askama::Template)]
#[template(path = "login_page/login_form.html")]
pub struct LoginForm {
    pub username: String,
    pub error_message: Option<String>,
}
