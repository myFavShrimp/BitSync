use maud::Render;

use super::base::LoggedInDocument;

pub struct ErrorPage {
    pub error_message: String,
}

impl<E> From<E> for ErrorPage
where
    E: std::error::Error,
{
    fn from(error: E) -> Self {
        let mut error_message = format!("Error: {}", error);
        let mut previous_error = error.source();

        if previous_error.is_some() {
            error_message.push_str("\nCaused by:");
        }

        while let Some(current_error) = previous_error {
            error_message.push_str(&format!("\n    {}", current_error));
            previous_error = current_error.source();
        }

        Self { error_message }
    }
}

impl Render for ErrorPage {
    fn render(&self) -> maud::Markup {
        LoggedInDocument(maud::html!(
            main {
                h1 { "An unexpected error occurred" }
                pre { (self.error_message) }
            }
        ))
        .render()
    }
}
