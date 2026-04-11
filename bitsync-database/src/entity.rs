use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
    pub active_totp_secret: Option<Vec<u8>>,
    pub dangling_totp_secret: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TotpRecoveryCode {
    pub user_id: Uuid,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InviteToken {
    pub id: Uuid,
    pub is_admin: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "session_platform", rename_all = "lowercase")]
pub enum SessionPlatform {
    MacOs,
    Windows,
    Linux,
    IOs,
    Android,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "session_browser", rename_all = "lowercase")]
pub enum SessionBrowser {
    Chrome,
    Firefox,
    Safari,
    Edge,
    Opera,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub platform: SessionPlatform,
    pub browser: SessionBrowser,
    pub created_at: time::OffsetDateTime,
    pub last_seen_at: time::OffsetDateTime,
}
