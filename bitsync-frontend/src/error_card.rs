use hypertext::prelude::*;

pub struct ErrorCard {
    pub title: String,
    pub message: String,
}

impl Renderable for ErrorCard {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            div class=(crate::styles::error_card::ClassName::ERROR_CARD) {
                div class=(crate::styles::error_card::ClassName::BORDER) {}
                div class=(crate::styles::error_card::ClassName::CONTENT) {
                    h2 class=(crate::styles::error_card::ClassName::TITLE) { (self.title) }
                    p { (self.message) }
                }
            }
        }
        .render_to(buffer);
    }
}
