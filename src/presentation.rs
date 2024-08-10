pub mod models;
pub mod templates;

fn format_file_size(bytes: u64) -> String {
    size::Size::from_bytes(bytes)
        .format()
        .with_style(size::Style::Abbreviated)
        .to_string()
}
