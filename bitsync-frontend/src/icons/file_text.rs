use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/file-text.svg");

pub struct FileText;

impl Renderable for FileText {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
