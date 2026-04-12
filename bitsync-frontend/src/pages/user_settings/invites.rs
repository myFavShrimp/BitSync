use bitsync_database::entity::InviteToken;
use bitsync_routes::TypedPath;
use hypertext::prelude::*;

use crate::{Component, error_banner::OptionalErrorBanner};

pub enum InvitesDisplayError {
    InternalServerError,
}

impl InvitesDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::InternalServerError => "An internal server error occurred",
        }
    }
}

static INVITE_LIST_ID: &str = "invite-list";

pub struct InvitesTabContent {
    pub invite_tokens: Vec<InviteToken>,
}

impl Renderable for InvitesTabContent {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            div class=(crate::styles::modal::ClassName::MODAL_BODY) {
                p class=(crate::styles::modal::ClassName::MODAL_DESCRIPTION) {
                    "Invite codes let new users register on this instance. Each code is single-use and can be revoked at any time."
                    // TODO: figure out if creation of admin tokens is a necessary feature
                    // " Admin invites grant administrator privileges to the user that redeems them."
                }

                (InviteList {
                    invite_tokens: self.invite_tokens.clone(),
                    error: None,
                })

                div class=(crate::styles::user_settings_page::ClassName::INVITES_ACTIONS) {
                    button
                        class=(
                            crate::styles::button::ClassName::BUTTON, " ",
                            crate::styles::button::ClassName::BUTTON_PRIMARY,
                        )
                        data-init=(format!("this.fetch = fetch('{}', {{ method: 'POST' }})", bitsync_routes::PostUserSettingsInviteTokenCreate))
                        data-on-click__throttle.1s="this.fetch.trigger()"
                        data-effect=(format!(
                            "
                                clearTimeout(this._lt),
                                this.fetch.state() === 'pending'
                                    ? this._lt = setTimeout(
                                        () => this.classList.add('{loading}'),
                                        200,
                                    ) : (
                                        this.classList.remove('{loading}'),
                                        this.disabled = false
                                    )
                            ",
                            loading = crate::styles::button::ClassName::BUTTON_LOADING,
                        ))
                    {
                        div class=(crate::styles::button::ClassName::BUTTON_SPINNER) {}
                        (crate::icons::plus::Plus)
                        "Add Invite"
                    }
                }
            }
        }
        .render_to(buffer);
    }
}

pub struct InviteList {
    pub invite_tokens: Vec<InviteToken>,
    pub error: Option<InvitesDisplayError>,
}

impl Component for InviteList {
    fn id(&self) -> String {
        INVITE_LIST_ID.to_owned()
    }
}

impl Renderable for InviteList {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        maud! {
            div
                id=(self.id())
                class=(crate::styles::user_settings_page::ClassName::INVITE_LIST)
            {
                OptionalErrorBanner message=(self.error.as_ref().map(|error| error.message().to_owned()));

                @if self.invite_tokens.is_empty() {
                    p class=(crate::styles::user_settings_page::ClassName::INVITE_EMPTY) {
                        "No invite codes yet. Generate one to invite a new user."
                    }
                } @else {
                    @for invite_token in &self.invite_tokens {
                        (InviteItem { invite_token: invite_token.clone() })
                    }
                }
            }
        }
        .render_to(buffer);
    }
}

struct InviteItem {
    invite_token: InviteToken,
}

impl Renderable for InviteItem {
    fn render_to(&self, buffer: &mut hypertext::Buffer) {
        let token = self.invite_token.id.to_string();

        let invite_path = bitsync_routes::GetRegisterPage
            .with_query_params(bitsync_routes::GetRegisterPageQueryParameters {
                token: Some(self.invite_token.id.to_string()),
            })
            .to_string();

        let delete_url = bitsync_routes::PostUserSettingsInviteTokenDelete {
            invite_token_id: self.invite_token.id,
        }
        .to_string();

        maud! {
            div class=(crate::styles::user_settings_page::ClassName::INVITE_ITEM) {
                div class=(crate::styles::user_settings_page::ClassName::INVITE_TOKEN_WRAPPER) {
                    pre
                        class=(crate::styles::user_settings_page::ClassName::INVITE_ID)
                        data-init=(format!(
                            "this.updateOverflow = createHorizontalOverflowHandler(this, '{overflow_left}', '{overflow_right}'), this.updateOverflow()",
                            overflow_left = crate::styles::user_settings_page::ClassName::OVERFLOW_LEFT,
                            overflow_right = crate::styles::user_settings_page::ClassName::OVERFLOW_RIGHT,
                        ))
                        data-on-scroll="this.updateOverflow()"
                    {
                        (token)
                    }
                }

                @if self.invite_token.is_admin {
                    span class=(crate::styles::user_settings_page::ClassName::INVITE_ADMIN_BADGE) {
                        "Admin"
                    }
                }

                div class=(crate::styles::user_settings_page::ClassName::INVITE_ACTIONS_GROUP) {
                    button
                        type="button"
                        class=(crate::styles::user_settings_page::ClassName::INVITE_COPY)
                        title="Copy invite link"
                        data-init=(format!(
                            "this.inviteLink = window.location.origin + '{invite_path}'"
                        ))
                        data-on-click=(format!(
                            "navigator.clipboard.writeText(this.inviteLink), this.classList.add('{copied}'), setTimeout(() => this.classList.remove('{copied}'), 1200)",
                            copied = crate::styles::user_settings_page::ClassName::COPIED,
                        ))
                    {
                        span class=(crate::styles::user_settings_page::ClassName::INVITE_COPY_ICON_DEFAULT) {
                            (crate::icons::link::Link)
                        }
                        span class=(crate::styles::user_settings_page::ClassName::INVITE_COPY_ICON_COPIED) {
                            (crate::icons::check::Check)
                        }
                    }

                    form
                        data-hijack
                        action=(delete_url)
                        method="POST"
                    {
                        button
                            type="submit"
                            class=(crate::styles::user_settings_page::ClassName::INVITE_REVOKE)
                            title="Revoke invite"
                        {
                            (crate::icons::circle_x::CircleX)
                        }
                    }
                }
            }
        }
        .render_to(buffer);
    }
}
