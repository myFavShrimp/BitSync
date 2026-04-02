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

                    script src="/static/external/hyperstim.js" type="module" {}

                    link href="/static/external/css/reset.css" rel="stylesheet" type="text/css";
                    link href="/static/external/css/Noto Sans.css" rel="stylesheet" type="text/css";
                    script src="/static/js/dialog-helper.js" defer {}

                    style { (crate::styles::base::STYLE_SHEET) }
                    style { (crate::styles::modal::STYLE_SHEET) }
                    style { (crate::styles::error_modal::STYLE_SHEET) }
                    style { (crate::styles::error_banner::STYLE_SHEET) }
                    style { (crate::styles::toast::STYLE_SHEET) }
                }
                body {
                    header {
                        a class=(crate::styles::base::ClassName::HEADER_LOGO) href=(bitsync_routes::GetFilesHomePage.to_string()) {
                            (crate::icons::logo::Logo::default())
                        }
                        nav {
                            @if cfg!(debug_assertions) {
                                div class=(crate::styles::base::ClassName::SEARCH_CONTAINER) {
                                    (crate::icons::search::Search)
                                    input type="text" placeholder=("Search files and folders...");
                                }
                            }
                            button class=(crate::styles::base::ClassName::NAV_MENU_BUTTON) popovertarget="nav-menu" title="Menu" {
                                (crate::icons::menu::Menu)
                            }
                            div class=(format!("{} {}", crate::styles::base::ClassName::CONTEXT_MENU, crate::styles::base::ClassName::NAV_CONTEXT_MENU)) id="nav-menu" popover {
                                a class=(crate::styles::base::ClassName::CONTEXT_MENU_ITEM) href=(bitsync_routes::GetUserSettingsPage.to_string()) {
                                    (crate::icons::bolt::Bolt)
                                    span { "Settings" }
                                }
                                div class=(crate::styles::base::ClassName::CONTEXT_MENU_DIVIDER) {}
                                a class=(format!("{} {}", crate::styles::base::ClassName::CONTEXT_MENU_ITEM, crate::styles::base::ClassName::CONTEXT_MENU_ITEM_DANGER)) href=(bitsync_routes::GetLogoutAction.to_string()) {
                                    (crate::icons::log_out::LogOut)
                                    span { "Sign Out" }
                                }
                            }
                        }
                    }

                    (self.children)

                    div
                        id=(crate::toast::TOAST_CONTAINER_ID)
                        class=(crate::styles::toast::ClassName::TOAST_CONTAINER)
                        role="status"
                        aria-live="polite"
                    {}
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

                    script src="/static/external/hyperstim.js" type="module" {}

                    link rel="stylesheet" type="text/css" href="/static/external/css/reset.css";
                    link type="text/css" href="/static/external/css/Noto Sans.css" rel="stylesheet";

                    style { (crate::styles::base::STYLE_SHEET) }
                    style { (crate::styles::error_banner::STYLE_SHEET) }
                    style { (crate::styles::toast::STYLE_SHEET) }
                }
                body {
                    (self.children)

                    div
                        id=(crate::toast::TOAST_CONTAINER_ID)
                        class=(crate::styles::toast::ClassName::TOAST_CONTAINER)
                        role="status"
                        aria-live="polite"
                    {}
                }
            }
        }
        .render_to(buffer);
    }
}
