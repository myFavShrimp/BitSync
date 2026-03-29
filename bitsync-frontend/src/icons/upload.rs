use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/upload.svg");

pub struct Upload;

impl Renderable for Upload {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
