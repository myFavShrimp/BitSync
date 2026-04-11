use hypertext::prelude::*;

pub struct ErrorBanner {
    pub message: String,
}

impl Renderable for ErrorBanner {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            div class=(crate::styles::error_banner::ClassName::ERROR_BANNER) {
                div class=(crate::styles::error_banner::ClassName::BORDER) {}
                p { (self.message) }
            }
        }
        .render_to(buffer);
    }
}

pub struct OptionalErrorBanner {
    pub message: Option<String>,
}

impl Renderable for OptionalErrorBanner {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            @if let Some(message) = &self.message {
                ErrorBanner message=(message.clone());
            }
        }
        .render_to(buffer);
    }
}
