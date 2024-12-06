use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}
