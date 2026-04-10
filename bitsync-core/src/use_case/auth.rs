pub mod ensure_admin_bootstrap;
pub mod login;
pub mod logout;
pub mod redeem_invite_token;
pub mod registration;
pub mod resolve_session;
pub mod retrieve_totp_setup_data;
pub mod setup_totp;
pub mod verify_totp;

use bitsync_database::entity::{SessionBrowser, SessionPlatform};

#[derive(thiserror::Error, Debug)]
#[error("the invite token is invalid or has already been used")]
pub struct InvalidInviteTokenError;

pub fn parse_user_agent_platform(user_agent: &str) -> SessionPlatform {
    if user_agent.contains("iPhone") || user_agent.contains("iPad") {
        SessionPlatform::IOs
    } else if user_agent.contains("Android") {
        SessionPlatform::Android
    } else if user_agent.contains("Macintosh") || user_agent.contains("Mac OS") {
        SessionPlatform::MacOs
    } else if user_agent.contains("Windows") {
        SessionPlatform::Windows
    } else if user_agent.contains("Linux") {
        SessionPlatform::Linux
    } else {
        SessionPlatform::Unknown
    }
}

pub fn parse_user_agent_browser(user_agent: &str) -> SessionBrowser {
    if user_agent.contains("Edg/") {
        SessionBrowser::Edge
    } else if user_agent.contains("OPR/") || user_agent.contains("Opera") {
        SessionBrowser::Opera
    } else if user_agent.contains("Firefox/") || user_agent.contains("FxiOS/") {
        SessionBrowser::Firefox
    } else if user_agent.contains("CriOS/") || user_agent.contains("Chrome/") {
        SessionBrowser::Chrome
    } else if user_agent.contains("Safari/") {
        SessionBrowser::Safari
    } else {
        SessionBrowser::Unknown
    }
}
