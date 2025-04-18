pub mod error_modal;
pub mod htmx;
pub mod icons;
pub mod models;
pub mod pages;
pub mod styles;

pub use maud::Render;

fn format_file_size(bytes: u64) -> String {
    size::Size::from_bytes(bytes)
        .format()
        .with_style(size::Style::Abbreviated)
        .to_string()
}
