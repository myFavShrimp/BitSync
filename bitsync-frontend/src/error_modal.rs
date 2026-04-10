use hypertext::prelude::*;

pub struct ErrorModal {
    message: String,
    trace: Option<String>,
}

impl ErrorModal {
    pub fn with_message(message: String) -> Self {
        Self {
            message,
            trace: None,
        }
    }
}

impl<E> From<E> for ErrorModal
where
    E: std::error::Error,
{
    fn from(error: E) -> Self {
        let message = format!("Error: {}", error);

        let trace = error.source().map(|previous_error| {
            let mut trace = format!("Caused by: {previous_error}");
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

impl Renderable for ErrorModal {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            div {
                dialog
                    class=(crate::styles::error_modal::ClassName::MODAL)
                    data-init="this.showModal()"
                {
                    h1 { (self.message) }

                    @match &self.trace {
                        Some(trace) => {
                            pre { (trace) }
                        }
                        None => {}
                    }

                    button onclick="closeClosestDialog(this)" { ("close") }
                }
            }
        }
        .render_to(buffer);
    }
}
