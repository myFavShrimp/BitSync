use maud::Render;

pub struct LoggedInDocument(pub maud::Markup);

impl Render for LoggedInDocument {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (maud::DOCTYPE)
            html lang="en"  {
                head {
                    meta content="text/html; charset=UTF-8" http-equiv="Content-Type";
                    meta content="width=device-width,initial-scale=1.0" name="viewport";
                    script src="/static/external/htmx.js" {}
                    link href="/static/external/css/reset.css" rel="stylesheet" type="text/css";
                    link href="/static/external/css/Noto Sans.css" rel="stylesheet" type="text/css";
                    script src="/static/js/dialog-helper.js" defer {}
                    style {
                        (crate::styles::base::STYLE_SHEET)
                    }
                    style {
                        (crate::styles::error_modal::STYLE_SHEET)
                    }
                    meta name="htmx-config" content=(*crate::htmx::CONFIG);
                }
                body {
                    nav {
                        a class=(crate::styles::files_home_page::ClassName::LOGO) href=(bitsync_routes::GetFilesHomePage.to_string()) {
                            (crate::icons::logo::Logo::default())
                        }
                        a href=(bitsync_routes::GetUserSettingsPage.to_string()) {
                            "User Settings"
                        }
                        a href=(bitsync_routes::GetLogoutAction.to_string()) {
                            "Log out"
                        }
                    }

                    (self.0)
                }
            }
        }
    }
}

pub struct GuestDocument(pub maud::Markup);

impl Render for GuestDocument {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (maud::DOCTYPE)
            html lang="en" {
                head {
                    meta content="text/html; charset=UTF-8" http-equiv="Content-Type";
                    meta content="width=device-width,initial-scale=1.0" name="viewport";
                    script src="static/external/htmx.js" {}
                    link rel="stylesheet" type="text/css" href="static/external/css/reset.css";
                    link type="text/css" href="static/external/css/Noto Sans.css" rel="stylesheet";
                    style { (crate::styles::base::STYLE_SHEET) }
                }
                body {
                    (self.0)
                }
            }
        }
    }
}
