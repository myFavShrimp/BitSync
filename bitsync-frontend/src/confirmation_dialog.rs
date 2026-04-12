use hypertext::prelude::*;

pub struct ConfirmationDialog {
    pub title: String,
    pub message: String,
    pub confirm_label: String,
    pub action_url: String,
    pub is_danger: bool,
}

impl Renderable for ConfirmationDialog {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        let confirm_button_class = if self.is_danger {
            format!(
                "{} {}",
                crate::styles::button::ClassName::BUTTON,
                crate::styles::button::ClassName::BUTTON_DANGER,
            )
        } else {
            format!(
                "{} {}",
                crate::styles::button::ClassName::BUTTON,
                crate::styles::button::ClassName::BUTTON_PRIMARY,
            )
        };

        maud! {
            dialog
                class=(crate::styles::modal::ClassName::MODAL)
                data-init="this.showModal()"
                onclick="if (event.target === this) closeClosestDialogAndRemoveElement(this)"
            {
                div class=(crate::styles::modal::ClassName::MODAL_HEADER) {
                    h2 class=(crate::styles::modal::ClassName::MODAL_TITLE) { (self.title) }

                    button
                        class=(crate::styles::modal::ClassName::MODAL_CLOSE)
                        onclick="closeClosestDialogAndRemoveElement(this)"
                    {
                        (crate::icons::x::X)
                    }
                }
                div class=(crate::styles::modal::ClassName::MODAL_BODY) {
                    p class=(crate::styles::modal::ClassName::MODAL_DESCRIPTION) {
                        (self.message)
                    }
                }
                div class=(crate::styles::modal::ClassName::MODAL_ACTIONS) {
                    button
                        class=(crate::styles::button::ClassName::BUTTON)
                        onclick="closeClosestDialogAndRemoveElement(this)"
                    {
                        "Cancel"
                    }
                    button
                        class=(confirm_button_class)
                        data-init=(format!(
                            "this.fetch = fetch('{}', {{ method: 'POST' }})",
                            self.action_url,
                        ))
                        data-on-click="closeClosestDialogAndRemoveElement(this), this.fetch.trigger()"
                    {
                        (self.confirm_label)
                    }
                }
            }
        }
        .render_to(buffer);
    }
}
