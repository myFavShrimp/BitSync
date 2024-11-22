#[derive(askama::Template)]
#[template(path = "shared/error_modal.html")]
pub struct ErrorModal {
    message: String,
    trace: Option<String>,
}

impl<E> From<E> for ErrorModal
where
    E: std::error::Error,
{
    fn from(error: E) -> Self {
        let message = format!("Error: {}", error);

        let trace = error.source().map(|previous_error| {
            let mut trace = format!("Caused by: {previous_error}",);
            let mut trace_error = error.source();

            while let Some(current_error) = trace_error {
                trace.push_str(&format!("\n    {}", current_error));
                trace_error = current_error.source();
            }

            trace
        });

        Self { message, trace }
    }
}
