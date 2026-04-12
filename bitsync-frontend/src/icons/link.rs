use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/link.svg");

pub struct Link;

impl Renderable for Link {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
