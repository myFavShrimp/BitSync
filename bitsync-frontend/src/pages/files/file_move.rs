use hypertext::prelude::*;

use crate::Component;

use super::FilesHomePageElementId;

pub struct FileMoveDialog {
    pub action_url: String,
    pub source_path: String,
}

impl Component for FileMoveDialog {
    fn id(&self) -> String {
        FilesHomePageElementId::FileMoveDialog.to_str().to_owned()
    }
}

impl Renderable for FileMoveDialog {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            dialog
                class=(crate::styles::modal::ClassName::MODAL)
                id=(self.id())
                data-init="this.showModal()"
                onclick="if (event.target === this) closeClosestDialogAndRemoveElement(this)"
            {
                div class=(crate::styles::modal::ClassName::MODAL_HEADER) {
                    h2 class=(crate::styles::modal::ClassName::MODAL_TITLE) { "Move Item" }

                    button
                        class=(crate::styles::modal::ClassName::MODAL_CLOSE)
                        onclick="closeClosestDialogAndRemoveElement(this)"
                    {
                        (crate::icons::X::default())
                    }
                }
                form
                    data-hijack
                    action=(self.action_url)
                    method="POST"
                {
                    div class=(crate::styles::modal::ClassName::MODAL_BODY) {
                        label class=(crate::styles::modal::ClassName::FORM_LABEL) {
                            "Destination Path"

                            input
                                class=(crate::styles::base::ClassName::FORM_CONTROL)
                                type="text"
                                name="destination_path"
                                value=(self.source_path)
                                placeholder="Enter destination path";
                        }
                    }
                    div class=(crate::styles::modal::ClassName::MODAL_ACTIONS) {
                        button
                            type="button"
                            class=(crate::styles::button::ClassName::BUTTON)
                            onclick="closeClosestDialogAndRemoveElement(this)"
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
                            "Move"
                        }
                    }
                }
            }
        }
        .render_to(buffer);
    }
}
