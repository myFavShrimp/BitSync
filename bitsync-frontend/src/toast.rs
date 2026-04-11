use hypertext::prelude::*;

pub static TOAST_CONTAINER_ID: &str = "toast-container";
pub static TOAST_CONTAINER_SELECTOR: &str = "#toast-container";

static TOAST_AUTO_DISMISS_MS: u32 = 5000;

pub enum ToastKind {
    Success,
    Error,
}

pub struct Toast {
    message: String,
    kind: ToastKind,
}

impl Toast {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind: ToastKind::Success,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind: ToastKind::Error,
        }
    }

    fn kind_class(&self) -> &str {
        match self.kind {
            ToastKind::Success => crate::styles::toast::ClassName::TOAST_SUCCESS,
            ToastKind::Error => crate::styles::toast::ClassName::TOAST_ERROR,
        }
    }
}

impl Renderable for Toast {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        let dismissing_class = crate::styles::toast::ClassName::TOAST_DISMISSING;

        maud! {
            div
                class=(
                    crate::styles::toast::ClassName::TOAST, " ",
                    self.kind_class(),
                )
                // popover needs to be repropagated. otherwise it will be behind dialogs/backdrop
                data-init=(format!(r"
                    this.parentElement.hidePopover(),
                    this.parentElement.showPopover(),
                    setTimeout(() => this.classList.add('{dismissing_class}'), {TOAST_AUTO_DISMISS_MS})
                "))
                data-on-animationend=(format!(
                    "this.classList.contains('{dismissing_class}') && this.remove()"
                ))
            {
                div class=(crate::styles::toast::ClassName::TOAST_INDICATOR) {}
                span { (self.message) }
            }
        }
        .render_to(buffer);
    }
}
