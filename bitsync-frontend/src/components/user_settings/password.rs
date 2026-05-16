use hypertext::prelude::*;

use crate::{Component, error_banner::OptionalErrorBanner};

pub enum PasswordDisplayError {
    InternalServerError,
    InvalidCurrentPassword,
    NewPasswordsMismatch,
    EmptyNewPassword,
}

impl PasswordDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::InternalServerError => "An internal server error occurred",
            Self::InvalidCurrentPassword => "Current password is incorrect",
            Self::NewPasswordsMismatch => "New passwords do not match",
            Self::EmptyNewPassword => "Password cannot be empty",
        }
    }
}

pub struct PasswordTabContent {
    pub error: Option<PasswordDisplayError>,
}

impl Component for PasswordTabContent {
    fn id(&self) -> String {
        "password-tab-content".to_owned()
    }
}

impl Renderable for PasswordTabContent {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            form
                id=(self.id())
                data-hijack
                action=(bitsync_routes::PostUserSettingsChangePassword.to_string())
                method="POST"
            {
                div class=(crate::styles::modal::ClassName::MODAL_BODY) {
                    p class=(crate::styles::modal::ClassName::MODAL_DESCRIPTION) {
                        "Update your password below. Changing your password will sign out all other active sessions."
                    }

                    OptionalErrorBanner message=(self.error.as_ref().map(|error| error.message().to_owned()));

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
                            crate::styles::button::ClassName::BUTTON, " ",
                            crate::styles::button::ClassName::BUTTON_PRIMARY
                        )
                        data-effect=(format!(
                            "handleButtonLoading(this, this.form.hsFetch, '{loading}')",
                            loading = crate::styles::button::ClassName::BUTTON_LOADING,
                        ))
                    {
                        div class=(crate::styles::button::ClassName::BUTTON_SPINNER) {}
                        "Change Password"
                    }
                }
            }
        }
        .render_to(buffer);
    }
}
