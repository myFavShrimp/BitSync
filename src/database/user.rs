use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
    pub color_palette: Option<String>,
}

impl User {
    pub async fn create(
        connection: &PgPool,
        username: &str,
        password: &str,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"INSERT INTO "user" (username, password) VALUES ($1, $2) RETURNING *"#,
            username,
            password
        )
        .fetch_one(connection)
        .await
    }

    pub async fn find_by_ids(connection: &PgPool, ids: &[Uuid]) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(User, r#"SELECT * FROM "user" WHERE id = ANY($1)"#, ids)
            .fetch_all(connection)
            .await
    }

    pub async fn find_by_id(connection: &PgPool, id: &Uuid) -> Result<User, sqlx::Error> {
        sqlx::query_as!(User, r#"SELECT * FROM "user" WHERE id = $1"#, id)
            .fetch_one(connection)
            .await
    }

    pub async fn find_all(connection: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(User, r#"SELECT * FROM "user""#)
            .fetch_all(connection)
            .await
    }

    pub async fn find_by_username(
        connection: &PgPool,
        username: &str,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT * FROM "user" WHERE username = $1"#,
            username,
        )
        .fetch_one(connection)
        .await
    }

    pub async fn update_password(
        connection: &PgPool,
        user_id: &Uuid,
        new_password: &str,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"UPDATE "user" SET password = $2 WHERE id = $1 RETURNING *"#,
            user_id,
            new_password,
        )
        .fetch_one(connection)
        .await
    }
}
