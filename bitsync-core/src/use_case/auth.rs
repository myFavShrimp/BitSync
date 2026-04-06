pub mod ensure_admin_bootstrap;
pub mod login;
pub mod redeem_invite_token;
pub mod registration;
pub mod resolve_session;
pub mod retrieve_totp_setup_data;
pub mod setup_totp;
pub mod verify_totp;

use bitsync_database::entity::SessionPlatform;

#[derive(thiserror::Error, Debug)]
#[error("the invite token is invalid or has already been used")]
pub struct InvalidInviteTokenError;

pub fn parse_navigator_platform(value: &str) -> SessionPlatform {
    let lowercase = value.to_lowercase();
    if lowercase.contains("mac") {
        SessionPlatform::MacOs
    } else if lowercase.contains("win") {
        SessionPlatform::Windows
    } else if lowercase.contains("linux") {
        SessionPlatform::Linux
    } else if lowercase.contains("iphone") || lowercase.contains("ipad") {
        SessionPlatform::IOs
    } else {
        SessionPlatform::Unknown
    }
}
