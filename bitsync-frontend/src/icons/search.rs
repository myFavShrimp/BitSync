use hypertext::prelude::*;

static SVG: &str = include_str!("../../../static/svg/search.svg");

pub struct Search;

impl Renderable for Search {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        hypertext::Raw::dangerously_create(SVG).render_to(buffer);
    }
}
