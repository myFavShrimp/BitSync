use hypertext::prelude::*;

pub static DIALOG_WRAPPER_ID: &str = "dialog-wrapper";
pub static DIALOG_WRAPPER_SELECTOR: &str = "#dialog-wrapper";

pub struct LoggedInDocument<R: Renderable> {
    pub current_path: Option<String>,
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
                    script src="/static/js/overflow-helper.js" defer {}

                    style { (crate::styles::base::STYLE_SHEET) }
                    style { (crate::styles::modal::STYLE_SHEET) }
                    style { (crate::styles::error_modal::STYLE_SHEET) }
                    style { (crate::styles::error_banner::STYLE_SHEET) }
                    style { (crate::styles::search_launcher::STYLE_SHEET) }
                    style { (crate::styles::toast::STYLE_SHEET) }
                }
                body {
                    header {
                        a class=(crate::styles::base::ClassName::HEADER_LOGO) href=(bitsync_routes::GetFilesHomePage.to_string()) {
                            (crate::icons::logo::Logo::default())
                        }
                        nav {
                            button
                                class=(crate::styles::base::ClassName::SEARCH_BUTTON)
                                title="Search"
                                onclick="openDialogModalById('search-launcher')"
                            {
                                (crate::icons::search::Search)
                                span { "Search files and folders..." }
                            }

                            dialog
                                class=(crate::styles::search_launcher::ClassName::SEARCH_LAUNCHER)
                                id="search-launcher"
                                onclick="if (event.target === this) this.close()"
                            {
                                button
                                    class=(crate::styles::search_launcher::ClassName::CLOSE_BUTTON)
                                    onclick="closeClosestDialog(this)"
                                {
                                    "×"
                                }

                                form
                                    data-hijack
                                    action=(bitsync_routes::GetSearch.to_string())
                                    method="GET"
                                {
                                    div class=(crate::styles::search_launcher::ClassName::INPUT_WRAPPER) {
                                        (crate::icons::search::Search)
                                        @if let Some(path) = &self.current_path {
                                            input
                                                type="hidden"
                                                name="path"
                                                value=(path);
                                        }
                                        input
                                            class=(crate::styles::search_launcher::ClassName::INPUT)
                                            type="text"
                                            name="query"
                                            placeholder="Search files and folders..."
                                            autocomplete="off"
                                            autofocus
                                            data-on-input__debounce.300ms="this.form.requestSubmit()";
                                    }
                                }
                                div
                                    class=(crate::styles::search_launcher::ClassName::RESULTS)
                                    id="search-results"
                                {}
                            }


                            button class=(crate::styles::base::ClassName::NAV_MENU_BUTTON) popovertarget="nav-menu" title="Menu" {
                                (crate::icons::menu::Menu)
                            }
                            div class=(format!("{} {}", crate::styles::base::ClassName::CONTEXT_MENU, crate::styles::base::ClassName::NAV_CONTEXT_MENU)) id="nav-menu" popover {
                                button
                                    class=(crate::styles::base::ClassName::CONTEXT_MENU_ITEM)
                                    data-init=(format!("this.fetch = fetch('{}')", bitsync_routes::GetUserSettingsPage.to_string()))
                                    data-on-click="closeClosestPopover(this), this.fetch.trigger()"
                                {
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

                    div id=(DIALOG_WRAPPER_ID) {}

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
