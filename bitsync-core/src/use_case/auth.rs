pub mod ensure_admin_bootstrap;
pub mod login;
pub mod redeem_invite_token;
pub mod registration;
pub mod retrieve_totp_setup_data;
pub mod setup_totp;
pub mod verify_totp;

#[derive(thiserror::Error, Debug)]
#[error("the invite token is invalid or has already been used")]
pub struct InvalidInviteTokenError;
