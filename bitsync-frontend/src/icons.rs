use std::marker::PhantomData;

use hypertext::Renderable;

pub trait IconKind {
    const SVG: &'static str;
}

#[derive(Default)]
pub struct Icon<I: IconKind> {
    class: Option<&'static str>,
    _marker: PhantomData<I>,
}

impl<I: IconKind> Icon<I> {
    pub fn with_class(class: &'static str) -> Self {
        Self {
            class: Some(class),
            _marker: PhantomData,
        }
    }
}

fn insert_class(svg: &'static str, class: &'static str) -> String {
    svg.replacen("<svg", &format!(r#"<svg class="{}""#, class), 1)
}

impl<I: IconKind> Renderable for Icon<I> {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        let svg = if let Some(class) = self.class {
            insert_class(I::SVG, class)
        } else {
            I::SVG.to_owned()
        };

        hypertext::Raw::dangerously_create(svg).render_to(buffer);
    }
}

macro_rules! icons {
    ( $( $name:ident: $path:literal, )* ) => {
        pub mod _icons {
            $(
                #[derive(Default)]
                pub struct $name;
                impl super::IconKind for $name {
                    const SVG: &'static str = include_str!($path);
                }
            )*
        }

        $(
            pub type $name = Icon<_icons::$name>;
        )*
    };
    ( $( $name:ident: $path:literal ),* ) => {
        icons! { $( $name: $path, )* }
    }
}

icons! {
    Bolt: "../../static/svg/bolt.svg",
    Check: "../../static/svg/check.svg",
    ChevronLeft: "../../static/svg/chevron-left.svg",
    ChevronRight: "../../static/svg/chevron-right.svg",
    CircleX: "../../static/svg/circle-x.svg",
    Cloudy: "../../static/svg/cloudy.svg",
    EllipsisVertical: "../../static/svg/ellipsis-vertical.svg",
    FileText: "../../static/svg/file-text.svg",
    Folder: "../../static/svg/folder.svg",
    FolderPlus: "../../static/svg/folder-plus.svg",
    Link: "../../static/svg/link.svg",
    LogOut: "../../static/svg/log-out.svg",
    Logo: "../../static/svg/logo.svg",
    Menu: "../../static/svg/menu.svg",
    Plus: "../../static/svg/plus.svg",
    Search: "../../static/svg/search.svg",
    Upload: "../../static/svg/upload.svg",
    X: "../../static/svg/x.svg",
}
