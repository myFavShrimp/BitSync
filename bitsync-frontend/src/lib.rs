pub mod error_banner;
pub mod error_modal;
pub mod icons;
pub mod models;
pub mod pages;
pub mod styles;
mod totp;

use hypertext::Renderable;

pub trait Render: hypertext::Renderable {
    fn render(&self) -> String {
        Renderable::render(&self).into_inner()
    }
}

impl<T> Render for T where T: Renderable {}

fn format_file_size(bytes: u64) -> String {
    size::Size::from_bytes(bytes)
        .format()
        .with_style(size::Style::Abbreviated)
        .to_string()
}
