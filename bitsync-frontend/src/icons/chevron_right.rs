use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/chevron-right.svg");

pub struct ChevronRight;

impl Renderable for ChevronRight {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
