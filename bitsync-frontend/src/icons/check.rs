use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/check.svg");

pub struct Check;

impl Renderable for Check {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
