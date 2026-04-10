use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/circle-x.svg");

pub struct CircleX;

impl Renderable for CircleX {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
