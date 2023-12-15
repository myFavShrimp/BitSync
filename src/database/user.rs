use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}

impl User {
    pub async fn find_by_ids(connection: &PgPool, ids: &[Uuid]) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(User, r#"SELECT * FROM "user" WHERE id = ANY($1)"#, ids)
            .fetch_all(connection)
            .await
    }
}
