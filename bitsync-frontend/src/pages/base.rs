use hypertext::prelude::*;

pub struct LoggedInDocument<R: Renderable> {
    pub children: R,
}

impl<R: Renderable> Renderable for LoggedInDocument<R> {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            !DOCTYPE
            html lang="en"  {
                head {
                    meta content="text/html; charset=UTF-8" http-equiv="Content-Type";
                    meta content="width=device-width,initial-scale=1.0" name="viewport";

                    script src="/static/external/hyperstim.js" {}

                    link href="/static/external/css/reset.css" rel="stylesheet" type="text/css";
                    link href="/static/external/css/Noto Sans.css" rel="stylesheet" type="text/css";

                    script src="/static/js/dialog-helper.js" defer {}

                    style { (crate::styles::base::STYLE_SHEET) }
                    style { (crate::styles::error_modal::STYLE_SHEET) }
                    style { (crate::styles::error_banner::STYLE_SHEET) }
                }
                body {
                    nav {
                        a class=(crate::styles::files_home_page::ClassName::LOGO) href=(bitsync_routes::GetFilesHomePage.to_string()) {
                            // (crate::icons::logo::Logo::default()) TODO
                        }
                        a href=(bitsync_routes::GetUserSettingsPage.to_string()) {
                            "User Settings"
                        }
                        a href=(bitsync_routes::GetLogoutAction.to_string()) {
                            "Log out"
                        }
                    }

                    (self.children)
                }
            }
        }.render_to(buffer);
    }
}

pub struct GuestDocument<R: Renderable> {
    pub children: R,
}

impl<R: Renderable> Renderable for GuestDocument<R> {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            !DOCTYPE
            html lang="en" {
                head {
                    meta content="text/html; charset=UTF-8" http-equiv="Content-Type";
                    meta content="width=device-width,initial-scale=1.0" name="viewport";

                    script src="/static/external/hyperstim.js" {}

                    link rel="stylesheet" type="text/css" href="/static/external/css/reset.css";
                    link type="text/css" href="/static/external/css/Noto Sans.css" rel="stylesheet";

                    style { (crate::styles::base::STYLE_SHEET) }
                    style { (crate::styles::error_banner::STYLE_SHEET) }
                }
                body {
                    (self.children)
                }
            }
        }
        .render_to(buffer);
    }
}
