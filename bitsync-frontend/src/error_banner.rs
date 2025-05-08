pub fn error_banner(message: &str) -> maud::Markup {
    maud::html! {
        div class=(crate::styles::error_banner::ClassName::ERROR_BANNER) {
            div class=(crate::styles::error_banner::ClassName::BORDER) {}
            p { (message) }
        }
    }
}

pub fn optional_error_banner(message: &Option<String>) -> maud::Markup {
    maud::html! {
        @if let Some(message) = message {
            (error_banner(message))
        }
    }
}
