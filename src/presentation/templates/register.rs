#[derive(askama::Template, Default)]
#[template(path = "register.html")]
pub struct Register {
    pub username: Option<String>,
    pub error_message: Option<String>,
}
