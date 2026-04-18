use hypertext::prelude::*;

use crate::{error_card::ErrorCard, pages::base::AuthDocument};

pub struct SuspendedPage;

impl Renderable for SuspendedPage {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            AuthDocument {
                style { (crate::styles::register_page::STYLE_SHEET) }

                (crate::icons::Logo::with_class(crate::styles::register_page::ClassName::LOGO))

                main {
                    div class=(crate::styles::register_page::ClassName::FORM) {
                        div class=(crate::styles::register_page::ClassName::TOTP_HEADER) {
                            h1 { "Account Suspended" }
                            p { "Your account has been suspended by an administrator. You are unable to access BitSync at this time." }
                        }

                        ErrorCard
                            title=("What does this mean?".to_owned())
                            message=("Your data is preserved, but you cannot log in or use any features until an administrator restores your access. If you believe this is a mistake, please contact your administrator.".to_owned());

                        a
                            class=(crate::styles::button::ClassName::BUTTON)
                            href=(bitsync_routes::GetLogoutAction.to_string())
                        {
                            "Sign Out"
                        }
                    }
                }
            }
        }
        .render_to(buffer);
    }
}
