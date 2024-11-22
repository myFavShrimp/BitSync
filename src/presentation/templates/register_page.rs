#[derive(askama::Template, Default)]
#[template(path = "register_page.html")]
pub struct RegisterPage {
    pub username: Option<String>,
    pub error_message: Option<String>,
}
