use bitsync_core::use_case::auth::setup_totp::SetupTotpResult;
use bitsync_routes::TypedPath;
use hypertext::prelude::*;

use crate::{
    Component, error_banner::OptionalErrorBanner, error_card::ErrorCard, pages::base::AuthDocument,
    totp::totp_qr_src,
};

pub enum RegistrationDisplayError {
    UsernameTaken,
    InvalidInviteToken,
    EmptyPassword,
    InternalServerError,
}

impl RegistrationDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::UsernameTaken => "The username is already taken",
            Self::InvalidInviteToken => "The invite token is invalid or has already been used",
            Self::EmptyPassword => "Password cannot be empty",
            Self::InternalServerError => "An internal server error occurred",
        }
    }
}

pub enum TotpSetupDisplayError {
    InvalidCode,
    InternalServerError,
}

impl TotpSetupDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidCode => "The entered TOTP code is invalid",
            Self::InternalServerError => "An internal server error occurred",
        }
    }
}

pub enum RegisterPage {
    InviteTokenInput(InviteTokenForm),
    UserRegistration(RegisterForm),
    TotpSetup(TotpSetupForm),
}

impl Default for RegisterPage {
    fn default() -> Self {
        Self::InviteTokenInput(Default::default())
    }
}

impl Renderable for RegisterPage {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            AuthDocument {
                style { (crate::styles::register_page::STYLE_SHEET) }

                (crate::icons::Logo::with_class(crate::styles::register_page::ClassName::LOGO))

                p class=(crate::styles::register_page::ClassName::PAGE_HINT) {
                    @match &self {
                        Self::InviteTokenInput(..) => ("Enter your invite token to register"),
                        Self::UserRegistration(..) | Self::TotpSetup(..) => ("Create an account to get started"),
                    }
                }

                main {
                    @match &self {
                        Self::InviteTokenInput(form) => (form),
                        Self::UserRegistration(register_form) => (register_form),
                        Self::TotpSetup(totp_setup_form) => (totp_setup_form),
                    }
                }
            }
        }
        .render_to(buffer);
    }
}

#[derive(Default)]
pub struct InviteTokenForm {
    pub error: Option<InviteTokenDisplayError>,
}

impl Component for InviteTokenForm {
    fn id(&self) -> String {
        "invite-token-form".to_owned()
    }
}

pub enum InviteTokenDisplayError {
    InvalidToken,
    InternalServerError,
}

impl InviteTokenDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidToken => "The invite token is invalid or has already been used",
            Self::InternalServerError => "An internal server error occurred",
        }
    }
}

impl Renderable for InviteTokenForm {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            form
                id=(self.id())
                class=(crate::styles::register_page::ClassName::FORM)
                data-hijack
                action=(bitsync_routes::PostRedeemInviteToken.to_string())
                method="POST"
            {
                label class=(crate::styles::register_page::ClassName::INPUT_WRAPPER) {
                    "Invite Token"

                    div class=(crate::styles::register_page::ClassName::INPUT) {
                        input
                            class=(crate::styles::base::ClassName::FORM_CONTROL)
                            name="token"
                            placeholder="Enter your invite token"
                            required;
                    }
                }

                OptionalErrorBanner message=(self.error.as_ref().map(|error| error.message().to_owned()));

                div class=(crate::styles::register_page::ClassName::ACTIONS) {
                    button
                        type="submit"
                        class=(crate::styles::button::ClassName::BUTTON)
                    {
                        "Continue"
                    }

                    a
                        href=(bitsync_routes::GetLoginPage.to_string())
                        class=(crate::styles::base::ClassName::TEXT_LINK)
                    {
                        "I already have an account"
                    }
                }
            }
        }
        .render_to(buffer);
    }
}

pub struct RegisterForm {
    pub token: String,
    pub username: Option<String>,
    pub error: Option<RegistrationDisplayError>,
}

impl Component for RegisterForm {
    fn id(&self) -> String {
        "register-form".to_owned()
    }
}

impl Renderable for RegisterForm {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            form
                id=(self.id())
                class=(crate::styles::register_page::ClassName::FORM)
                data-hijack
                action=(bitsync_routes::PostRegisterAction.with_query_params(bitsync_routes::PostRegisterActionQueryParameters { token: self.token.clone() }).to_string())
                method="POST"
            {
                label class=(crate::styles::register_page::ClassName::INPUT_WRAPPER) {
                    "Username"

                    div class=(crate::styles::register_page::ClassName::INPUT) {
                        input
                            class=(crate::styles::base::ClassName::FORM_CONTROL)
                            value=[&self.username]
                            name="username"
                            placeholder="Enter your username"
                            required;
                    }
                }
                label class=(crate::styles::register_page::ClassName::INPUT_WRAPPER) {
                    "Password"

                    div class=(crate::styles::register_page::ClassName::INPUT) {
                        input
                            class=(crate::styles::base::ClassName::FORM_CONTROL)
                            type="password"
                            placeholder="Enter your password"
                            required
                            name="password";
                    }
                }

                OptionalErrorBanner message=(self.error.as_ref().map(|error| error.message().to_owned()));

                div class=(crate::styles::register_page::ClassName::ACTIONS) {
                    button
                        type="submit"
                        class=(crate::styles::button::ClassName::BUTTON)
                    {
                        "Register"
                    }

                    a
                        href=(bitsync_routes::GetLoginPage.to_string())
                        class=(crate::styles::base::ClassName::TEXT_LINK)
                    {
                        "I already have an account"
                    }
                }
            }
        }
        .render_to(buffer);
    }
}

static REGISTER_TOTP_FORM_ID: &str = "register-totp-form";

#[derive(Default)]
pub struct TotpSetupForm {
    pub totp_secret_image_base64_img_src: String,
    pub totp_secret: String,
    pub error: Option<TotpSetupDisplayError>,
}

impl Component for TotpSetupForm {
    fn id(&self) -> String {
        REGISTER_TOTP_FORM_ID.to_owned()
    }
}

impl Renderable for TotpSetupForm {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            form
                id=(self.id())
                class=(crate::styles::register_page::ClassName::FORM)
                data-hijack
                action=(bitsync_routes::PostRegisterTotpSetupAction.to_string())
                method="POST"
            {
                div class=(crate::styles::register_page::ClassName::TOTP_HEADER) {
                    h1 {"Two-Factor Authentication Setup"}
                    p {"Scan the QR code with your authenticator app (Google Authenticator, Authy, etc.)"}
                }
                div class=(crate::styles::register_page::ClassName::TOTP_QR_WRAPPER) {
                    img src=(totp_qr_src(&self.totp_secret_image_base64_img_src));
                }
                details class=(crate::styles::register_page::ClassName::TOTP_SECRET) {
                    summary {
                        "Can't scan? Show manual entry code"
                    }
                    pre { code { (self.totp_secret) } }
                }

                hr;

                OptionalErrorBanner message=(self.error.as_ref().map(|error| error.message().to_owned()));

                label class=(crate::styles::register_page::ClassName::INPUT_WRAPPER) {
                    "TOTP Code"

                    div class=(crate::styles::register_page::ClassName::TOTP_INPUT_WRAPPER) {
                        input
                            class=(crate::styles::base::ClassName::FORM_CONTROL)
                            name="totp"
                            placeholder="Enter the 6-digit code"
                            required;

                        p
                            id="totp-timer"
                            class=(crate::styles::register_page::ClassName::TOTP_TIMER)
                        {
                            "30"
                        }

                        script {(hypertext::Raw::dangerously_create(r#"
                            setInterval(() => {
                                const totpTimer = document.querySelector('#totp-timer');
                                const time = 30 - (Math.floor(Date.now() / 1000) % 30);
                                totpTimer.textContent = time;
                                totpTimer.style.background = `conic-gradient(var(--timer-pie-color) ${time/30*360}deg, rgba(255, 255, 255, 0.1) 0deg)`;
                            }, 100);
                        "#))}
                    }
                }

                button
                    type="submit"
                    class=(crate::styles::button::ClassName::BUTTON)
                {
                    "send"
                }
            }
        }
        .render_to(buffer);
    }
}

pub struct TotpAlreadySetUpNotice;

impl Component for TotpAlreadySetUpNotice {
    fn id(&self) -> String {
        REGISTER_TOTP_FORM_ID.to_owned()
    }
}

impl Renderable for TotpAlreadySetUpNotice {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            div
                id=(self.id())
                class=(crate::styles::register_page::ClassName::FORM)
            {
                div class=(crate::styles::register_page::ClassName::TOTP_HEADER) {
                    h1 {"Two-Factor Authentication"}
                }

                ErrorCard
                    title=("Already active".to_owned())
                    message=("Your account is already protected by a two-factor authenticator. If you weren't the one who set it up, contact an administrator right away. Someone else may have access to your account.".to_owned());

                a
                    class=(crate::styles::button::ClassName::BUTTON)
                    href=(bitsync_routes::GetLogoutAction.to_string())
                {
                    "Sign out"
                }
            }
        }
        .render_to(buffer);
    }
}

pub struct TotpRecoveryCodesPrompt {
    recovery_codes: Vec<String>,
}

impl From<SetupTotpResult> for TotpRecoveryCodesPrompt {
    fn from(value: SetupTotpResult) -> Self {
        Self {
            recovery_codes: value.recovery_codes,
        }
    }
}

impl Component for TotpRecoveryCodesPrompt {
    fn id(&self) -> String {
        REGISTER_TOTP_FORM_ID.to_owned()
    }
}

impl Renderable for TotpRecoveryCodesPrompt {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            div
                id=(self.id())
                class=(crate::styles::register_page::ClassName::FORM)
            {
                div class=(crate::styles::register_page::ClassName::TOTP_HEADER) {
                    h1 {"Save Your Recovery Codes"}
                    p {"Your two-factor authentication is now active."}
                }

                p {
                    ("To ensure you don't lose access to your account, please save these recovery codes in a secure location.")
                }

                p {
                    ("If you ever lose access to your authenticator app, you can enter any of these codes in the TOTP field when signing in. Each code works only once.")
                }

                p {
                    ("These codes will only be shown now. If you navigate away without saving them, you'll need to generate new ones.")
                }

                details
                    class=(crate::styles::register_page::ClassName::TOTP_SECRET)
                    open
                {
                    summary {
                        "Recovery Codes"
                    }

                    div class=(crate::styles::register_page::ClassName::RECOVERY_CODES) {
                        @for recovery_code in &self.recovery_codes {
                            pre {
                                code {
                                    (recovery_code)
                                }
                            }
                        }
                    }
                }

                a
                    class=(crate::styles::button::ClassName::BUTTON)
                    href=(bitsync_routes::GetFilesHomePage.to_string())
                {
                    "Continue"
                }
            }
        }
        .render_to(buffer);
    }
}
