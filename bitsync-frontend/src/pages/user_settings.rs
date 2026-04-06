use hypertext::prelude::*;

pub static SETTINGS_DIALOG_ID: &str = "settings-dialog";

pub struct SettingsDialog;

impl Renderable for SettingsDialog {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            dialog
                class=(crate::styles::modal::ClassName::MODAL)
                id=(SETTINGS_DIALOG_ID)
                data-init="this.showModal()"
                onclick="if (event.target === this) closeClosestDialogAndRemoveElement(this)"
            {
                div class=(crate::styles::modal::ClassName::MODAL_HEADER) {
                    h2 class=(crate::styles::modal::ClassName::MODAL_TITLE) { "Settings" }
                    button class=(crate::styles::modal::ClassName::MODAL_CLOSE) onclick="closeClosestDialogAndRemoveElement(this)" { "×" }
                }
                div class=(crate::styles::modal::ClassName::TAB_BAR_WRAPPER) {
                    div
                        class=(crate::styles::modal::ClassName::TAB_BAR)
                        data-init=(format!(
                            "this.updateOverflow = createHorizontalOverflowHandler(this, '{overflow_left}', '{overflow_right}'), this.updateOverflow()",
                            overflow_left = crate::styles::modal::ClassName::OVERFLOW_LEFT,
                            overflow_right = crate::styles::modal::ClassName::OVERFLOW_RIGHT,
                        ))
                        data-on-scroll="this.updateOverflow()"
                    {
                        button class=(crate::styles::modal::ClassName::TAB, " ", crate::styles::modal::ClassName::ACTIVE) { "Password" }
                        button class=(crate::styles::modal::ClassName::TAB, " ", crate::styles::modal::ClassName::DISABLED) disabled { "TOTP" }
                        button class=(crate::styles::modal::ClassName::TAB, " ", crate::styles::modal::ClassName::DISABLED) disabled { "Sessions" }
                        button class=(crate::styles::modal::ClassName::TAB, " ", crate::styles::modal::ClassName::DISABLED) disabled { "Shares" }
                        button class=(crate::styles::modal::ClassName::TAB, " ", crate::styles::modal::ClassName::DISABLED) disabled { "TOTP" }
                        button class=(crate::styles::modal::ClassName::TAB, " ", crate::styles::modal::ClassName::DISABLED) disabled { "Sessions" }
                        button class=(crate::styles::modal::ClassName::TAB, " ", crate::styles::modal::ClassName::DISABLED) disabled { "Shares" }
                        button class=(crate::styles::modal::ClassName::TAB, " ", crate::styles::modal::ClassName::DISABLED) disabled { "TOTP" }
                        button class=(crate::styles::modal::ClassName::TAB, " ", crate::styles::modal::ClassName::DISABLED) disabled { "Sessions" }
                        button class=(crate::styles::modal::ClassName::TAB, " ", crate::styles::modal::ClassName::DISABLED) disabled { "Shares" }
                    }
                }
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
                            input class=(crate::styles::base::ClassName::FORM_CONTROL) type="password" name="current_password" placeholder="Enter your current password";
                        }
                        div class=(crate::styles::modal::ClassName::FORM_DIVIDER) {}
                        label class=(crate::styles::modal::ClassName::FORM_LABEL) {
                            "New Password"
                            input class=(crate::styles::base::ClassName::FORM_CONTROL) type="password" name="new_password" placeholder="Enter a new password";
                        }
                        label class=(crate::styles::modal::ClassName::FORM_LABEL) {
                            "Confirm New Password"
                            input class=(crate::styles::base::ClassName::FORM_CONTROL) type="password" name="new_password_repeated" placeholder="Repeat your new password";
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
        }
        .render_to(buffer);
    }
}
