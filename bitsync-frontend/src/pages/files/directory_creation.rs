use hypertext::prelude::*;

use crate::{Component, error_banner::OptionalErrorBanner};

use super::FilesHomePageElementId;

pub enum DirectoryCreationDisplayError {
    EmptyName,
    InvalidName,
    InvalidPath,
    InternalServerError,
}

impl DirectoryCreationDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::EmptyName => "Folder name must not be empty",
            Self::InvalidName => "Folder name must not contain path separators",
            Self::InvalidPath => "Path must not contain '..' segments",
            Self::InternalServerError => "An internal server error occurred",
        }
    }
}

static DIRECTORY_CREATION_FORM_ID: &str = "directory-creation-form";

pub struct DirectoryCreationForm {
    pub action_url: String,
    pub directory_name: Option<String>,
    pub error: Option<DirectoryCreationDisplayError>,
}

impl Component for DirectoryCreationForm {
    fn id(&self) -> String {
        DIRECTORY_CREATION_FORM_ID.to_owned()
    }
}

impl Renderable for DirectoryCreationForm {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            form
                id=(self.id())
                data-hijack
                action=(self.action_url)
                method="POST"
            {
                div class=(crate::styles::modal::ClassName::MODAL_BODY) {
                    label class=(crate::styles::modal::ClassName::FORM_LABEL) {
                        "Folder Name"

                        input
                            class=(crate::styles::base::ClassName::FORM_CONTROL)
                            type="text"
                            name="directory_name"
                            value=(self.directory_name.clone().unwrap_or_default())
                            placeholder="Enter folder name";
                    }

                    OptionalErrorBanner message=(self.error.as_ref().map(|error| error.message().to_owned()));
                }
                div class=(crate::styles::modal::ClassName::MODAL_ACTIONS) {
                    button
                        type="button"
                        class=(crate::styles::button::ClassName::BUTTON)
                        onclick="closeClosestDialog(this)"
                    {
                        "Cancel"
                    }
                    button
                        type="submit"
                        class=(
                            crate::styles::button::ClassName::BUTTON, " ",
                            crate::styles::button::ClassName::BUTTON_PRIMARY,
                        )
                        data-effect=(format!(
                            "handleButtonLoading(this, this.form.hsFetch, '{loading}')",
                            loading = crate::styles::button::ClassName::BUTTON_LOADING,
                        ))
                    {
                        div class=(crate::styles::button::ClassName::BUTTON_SPINNER) {}
                        "Create"
                    }
                }
            }
        }
        .render_to(buffer);
    }
}

pub struct DirectoryCreationDialog {
    pub action_url: String,
}

impl Component for DirectoryCreationDialog {
    fn id(&self) -> String {
        FilesHomePageElementId::DirectoryCreationDialog
            .to_str()
            .to_owned()
    }
}

impl Renderable for DirectoryCreationDialog {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            dialog
                class=(crate::styles::modal::ClassName::MODAL)
                id=(self.id())
                data-init="this.showModal()"
                onclick="if (event.target === this) closeClosestDialogAndRemoveElement(this)"
            {
                div class=(crate::styles::modal::ClassName::MODAL_HEADER) {
                    h2 class=(crate::styles::modal::ClassName::MODAL_TITLE) { "Create New Folder" }

                    button
                        class=(crate::styles::modal::ClassName::MODAL_CLOSE)
                        onclick="closeClosestDialogAndRemoveElement(this)"
                    {
                        (crate::icons::X::default())
                    }
                }
                (DirectoryCreationForm {
                    action_url: self.action_url.clone(),
                    directory_name: None,
                    error: None,
                })
            }
        }
        .render_to(buffer);
    }
}
