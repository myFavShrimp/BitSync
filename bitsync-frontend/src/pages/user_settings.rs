use bitsync_database::entity::Session;
use hypertext::prelude::*;
use uuid::Uuid;

use crate::Component;

pub mod password;
pub mod sessions;
pub mod totp;

use self::{password::PasswordTabContent, sessions::SessionsTabContent, totp::TotpTabContent};

pub static SETTINGS_DIALOG_ID: &str = "settings-dialog";
static SETTINGS_TAB_AREA_ID: &str = "settings-tab-area";

pub enum SettingsTab {
    Password,
    Sessions {
        sessions: Vec<Session>,
        current_session_id: Uuid,
    },
    Totp(TotpTabContent),
}

pub struct SettingsDialog {
    pub sessions: Vec<Session>,
    pub current_session_id: Uuid,
}

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

                    button
                        class=(crate::styles::modal::ClassName::MODAL_CLOSE)
                        onclick="closeClosestDialogAndRemoveElement(this)"
                    {
                        (crate::icons::x::X)
                    }
                }
                (SettingsTabArea {
                    active_tab: SettingsTab::Sessions {
                        sessions: self.sessions.clone(),
                        current_session_id: self.current_session_id,
                    },
                })
            }
        }
        .render_to(buffer);
    }
}

pub struct SettingsTabArea {
    pub active_tab: SettingsTab,
}

impl Component for SettingsTabArea {
    fn id(&self) -> String {
        SETTINGS_TAB_AREA_ID.to_owned()
    }
}

impl Renderable for SettingsTabArea {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        let is_password_active = matches!(self.active_tab, SettingsTab::Password);
        let is_sessions_active = matches!(self.active_tab, SettingsTab::Sessions { .. });
        let is_totp_active = matches!(self.active_tab, SettingsTab::Totp(..));

        fn tab_class(active: bool) -> String {
            if active {
                format!(
                    "{} {}",
                    crate::styles::modal::ClassName::TAB,
                    crate::styles::modal::ClassName::ACTIVE,
                )
            } else {
                crate::styles::modal::ClassName::TAB.to_owned()
            }
        }

        let password_tab_class = tab_class(is_password_active);
        let sessions_tab_class = tab_class(is_sessions_active);
        let totp_tab_class = tab_class(is_totp_active);

        maud! {
            div id=(self.id()) {
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
                        button
                            class=(sessions_tab_class)
                            data-init=(format!("this.fetch = fetch('{}')", bitsync_routes::GetUserSettingsSessionsTab))
                            data-on-click="this.fetch.trigger()"
                        {
                            "Sessions"
                        }
                        button
                            class=(
                                crate::styles::modal::ClassName::TAB, " ",
                                crate::styles::modal::ClassName::DISABLED,
                            )
                            disabled
                        {
                            "Shares"
                        }
                        button
                            class=(password_tab_class)
                            data-init=(format!("this.fetch = fetch('{}')", bitsync_routes::GetUserSettingsPasswordTab))
                            data-on-click="this.fetch.trigger()"
                        {
                            "Password"
                        }
                        button
                            class=(totp_tab_class)
                            data-init=(format!("this.fetch = fetch('{}')", bitsync_routes::GetUserSettingsTotpTab))
                            data-on-click="this.fetch.trigger()"
                        {
                            "TOTP"
                        }
                    }
                }
                @match &self.active_tab {
                    SettingsTab::Password => {
                        (PasswordTabContent { error: None })
                    }
                    SettingsTab::Sessions { sessions, current_session_id } => {
                        (SessionsTabContent {
                            sessions: sessions.clone(),
                            current_session_id: *current_session_id,
                        })
                    }
                    SettingsTab::Totp(content) => {
                        (content.clone())
                    }
                }
            }
        }
        .render_to(buffer);
    }
}
