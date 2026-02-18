use hypertext::prelude::*;

static LOGO_SVG: &str = include_str!("../../../static/svg/logo.svg");

#[derive(Default)]
pub struct Logo(&'static str);

impl Logo {
    pub fn with_class(class: &'static str) -> Self {
        Self(class)
    }
}

impl Renderable for Logo {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        let svg = if self.0.is_empty() {
            LOGO_SVG.to_owned()
        } else {
            LOGO_SVG.replacen("<svg", &format!(r#"<svg class="{}""#, self.0), 1)
        };
        hypertext::Raw::dangerously_create(svg).render_to(buffer);
    }
}
