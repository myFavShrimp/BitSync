use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/menu.svg");

pub struct Menu;

impl Renderable for Menu {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
