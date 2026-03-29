use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/folder.svg");

pub struct Folder;

impl Renderable for Folder {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
