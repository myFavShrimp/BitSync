use hypertext::prelude::*;

pub struct PasswordTabContent;

impl Renderable for PasswordTabContent {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            form
                data-hijack
                action=(bitsync_routes::PostUserSettingsChangePassword.to_string())
                method="POST"
            {
                div class=(crate::styles::modal::ClassName::MODAL_BODY) {
                    p class=(crate::styles::modal::ClassName::MODAL_DESCRIPTION) {
                        "Update your password below. Changing your password will sign out all other active sessions."
                    }

                    label class=(crate::styles::modal::ClassName::FORM_LABEL) {
                        "Current Password"

                        input
                            class=(crate::styles::base::ClassName::FORM_CONTROL)
                            type="password"
                            name="current_password"
                            placeholder="Enter your current password";
                    }

                    div class=(crate::styles::modal::ClassName::FORM_DIVIDER) {}

                    label class=(crate::styles::modal::ClassName::FORM_LABEL) {
                        "New Password"

                        input
                            class=(crate::styles::base::ClassName::FORM_CONTROL)
                            type="password"
                            name="new_password"
                            placeholder="Enter a new password";
                    }

                    label class=(crate::styles::modal::ClassName::FORM_LABEL) {
                        "Confirm New Password"

                        input
                            class=(crate::styles::base::ClassName::FORM_CONTROL)
                            type="password"
                            name="new_password_repeated"
                            placeholder="Repeat your new password";
                    }

                    button
                        type="submit"
                        class=(
                            crate::styles::base::ClassName::BUTTON, " ",
                            crate::styles::base::ClassName::BUTTON_PRIMARY
                        )
                    {
                        "Change Password"
                    }
                }
            }
        }
        .render_to(buffer);
    }
}
