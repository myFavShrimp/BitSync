use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/cloudy.svg");

pub struct Cloudy;

impl Renderable for Cloudy {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
