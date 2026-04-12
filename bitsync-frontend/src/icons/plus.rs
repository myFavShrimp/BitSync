use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/plus.svg");

pub struct Plus;

impl Renderable for Plus {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
