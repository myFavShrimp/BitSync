use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/log-out.svg");

pub struct LogOut;

impl Renderable for LogOut {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
