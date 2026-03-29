use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/chevron-left.svg");

pub struct ChevronLeft;

impl Renderable for ChevronLeft {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
