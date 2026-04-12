use bitsync_database::entity::{Session, SessionBrowser, SessionPlatform};
use hypertext::prelude::*;
use uuid::Uuid;

use crate::{Component, error_banner::OptionalErrorBanner};

pub enum SessionsDisplayError {
    InternalServerError,
    CannotTerminateCurrentSession,
}

impl SessionsDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::InternalServerError => "An internal server error occurred",
            Self::CannotTerminateCurrentSession => "Cannot terminate the current session",
        }
    }
}

static SESSION_LIST_ID: &str = "session-list";

fn platform_display_name(platform: &SessionPlatform) -> &'static str {
    match platform {
        SessionPlatform::MacOs => "macOS",
        SessionPlatform::Windows => "Windows",
        SessionPlatform::Linux => "Linux",
        SessionPlatform::IOs => "iOS",
        SessionPlatform::Android => "Android",
        SessionPlatform::Unknown => "Unknown",
    }
}

fn browser_display_name(browser: &SessionBrowser) -> &'static str {
    match browser {
        SessionBrowser::Chrome => "Chrome",
        SessionBrowser::Firefox => "Firefox",
        SessionBrowser::Safari => "Safari",
        SessionBrowser::Edge => "Edge",
        SessionBrowser::Opera => "Opera",
        SessionBrowser::Unknown => "Unknown App",
    }
}

fn session_display_name(session: &Session) -> String {
    format!(
        "{} on {}",
        browser_display_name(&session.browser),
        platform_display_name(&session.platform),
    )
}

fn format_datetime(datetime: &time::OffsetDateTime) -> String {
    let format = time::macros::format_description!(
        "[month repr:short] [day], [year], [hour repr:12]:[minute] [period]"
    );

    datetime
        .format(&format)
        // TODO: maybe find better way. Construct presentation type before?
        .unwrap_or_else(|_| "Unknown".to_owned())
}

pub struct SessionsTabContent {
    pub sessions: Vec<Session>,
    pub current_session_id: Uuid,
}

impl Renderable for SessionsTabContent {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        let has_other_sessions = self.sessions.len() > 1;

        maud! {
            div class=(crate::styles::modal::ClassName::MODAL_BODY) {
                p class=(crate::styles::modal::ClassName::MODAL_DESCRIPTION) {
                    "Devices currently signed in to your account."
                }
                (SessionList {
                    sessions: self.sessions.clone(),
                    current_session_id: self.current_session_id,
                    error: None,
                })
                @if has_other_sessions {
                    form
                        data-hijack
                        action=(bitsync_routes::PostTerminateAllOtherSessions.to_string())
                        method="POST"
                    {
                        button
                            type="submit"
                            class=(
                                crate::styles::button::ClassName::BUTTON, " ",
                                crate::styles::button::ClassName::BUTTON_DANGER,
                            )
                        {
                            "Revoke All Other Sessions"
                        }
                    }
                }
            }
        }
        .render_to(buffer);
    }
}

pub struct SessionList {
    pub sessions: Vec<Session>,
    pub current_session_id: Uuid,
    pub error: Option<SessionsDisplayError>,
}

impl Component for SessionList {
    fn id(&self) -> String {
        SESSION_LIST_ID.to_owned()
    }
}

impl Renderable for SessionList {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            div
                id=(self.id())
                class=(crate::styles::user_settings_page::ClassName::SESSION_LIST)
            {
                OptionalErrorBanner message=(self.error.as_ref().map(|error| error.message().to_owned()));

                @for session in &self.sessions {
                    // TODO move to use case?
                    @let is_current = session.id == self.current_session_id;

                    div class=(
                        if is_current {
                            format!(
                                "{} {}",
                                crate::styles::user_settings_page::ClassName::SESSION_ITEM,
                                crate::styles::user_settings_page::ClassName::SESSION_ITEM_CURRENT,
                            )
                        } else {
                            crate::styles::user_settings_page::ClassName::SESSION_ITEM.to_owned()
                        }
                    ) {
                        div class=(crate::styles::user_settings_page::ClassName::SESSION_INFO) {
                            div class=(crate::styles::user_settings_page::ClassName::SESSION_DEVICE) {
                                (session_display_name(session))
                                @if is_current {
                                    span class=(crate::styles::user_settings_page::ClassName::SESSION_BADGE) {
                                        "This device"
                                    }
                                }
                            }
                            div class=(crate::styles::user_settings_page::ClassName::SESSION_META) {
                                span { "Created " (format_datetime(&session.created_at)) }
                                span { "Last active " (format_datetime(&session.last_seen_at)) }
                            }
                        }
                        @if !is_current {
                            form
                                data-hijack
                                action=({
                                    let route = bitsync_routes::PostTerminateSession { session_id: session.id };
                                    route.to_string()
                                })
                                method="POST"
                            {
                                button
                                    type="submit"
                                    class=(crate::styles::user_settings_page::ClassName::SESSION_REVOKE)
                                    title="Revoke session"
                                {
                                    (crate::icons::circle_x::CircleX)
                                }
                            }
                        }
                    }
                }
            }
        }
        .render_to(buffer);
    }
}
