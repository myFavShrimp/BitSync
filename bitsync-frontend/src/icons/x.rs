use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/x.svg");

pub struct X;

impl Renderable for X {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
