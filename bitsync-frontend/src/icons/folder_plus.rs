use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/folder-plus.svg");

pub struct FolderPlus;

impl Renderable for FolderPlus {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
