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
                form
                    data-hijack
                    action=(bitsync_routes::PostUserSettingsChangePassword.to_string())
                    method="POST"
                {
                    div class=(crate::styles::modal::ClassName::MODAL_BODY) {
                        h3 class=(crate::styles::modal::ClassName::MODAL_SECTION_TITLE) { "Change Password" }
                        label class=(crate::styles::modal::ClassName::FORM_LABEL) {
                            "Current Password"
                            input class=(crate::styles::base::ClassName::FORM_CONTROL) type="password" name="current_password";
                        }
                        label class=(crate::styles::modal::ClassName::FORM_LABEL) {
                            "New Password"
                            input class=(crate::styles::base::ClassName::FORM_CONTROL) type="password" name="new_password";
                        }
                        label class=(crate::styles::modal::ClassName::FORM_LABEL) {
                            "Repeat New Password"
                            input class=(crate::styles::base::ClassName::FORM_CONTROL) type="password" name="new_password_repeated";
                        }
                    }
                    div class=(crate::styles::modal::ClassName::MODAL_ACTIONS) {
                        button type="button" class=(crate::styles::modal::ClassName::MODAL_BUTTON) onclick="closeClosestDialogAndRemoveElement(this)" { "Cancel" }
                        button type="submit" class=(format!("{} {}", crate::styles::modal::ClassName::MODAL_BUTTON, crate::styles::modal::ClassName::MODAL_BUTTON_PRIMARY)) { "Save" }
                    }
                }
            }
        }
        .render_to(buffer);
    }
}
