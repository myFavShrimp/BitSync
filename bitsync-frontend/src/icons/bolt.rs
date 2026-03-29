use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/bolt.svg");

pub struct Bolt;

impl Renderable for Bolt {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
