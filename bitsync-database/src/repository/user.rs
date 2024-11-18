use sqlx::PgExecutor;
use uuid::Uuid;

use crate::entity::User;

use super::QueryError;

pub async fn create<'e, E>(
    connection: E,
    username: &str,
    password: &str,
) -> Result<User, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        User,
        r#"INSERT INTO "user" (username, password) VALUES ($1, $2) RETURNING *"#,
        username,
        password
    )
    .fetch_one(connection)
    .await?)
}

pub async fn find_by_ids<'e, E>(executor: E, ids: &[Uuid]) -> Result<Vec<User>, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(
        sqlx::query_as!(User, r#"SELECT * FROM "user" WHERE id = ANY($1)"#, ids)
            .fetch_all(executor)
            .await?,
    )
}

pub async fn find_by_id<'e, E>(executor: E, id: &Uuid) -> Result<User, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(
        sqlx::query_as!(User, r#"SELECT * FROM "user" WHERE id = $1"#, id)
            .fetch_one(executor)
            .await?,
    )
}

pub async fn find_all<'e, E>(executor: E) -> Result<Vec<User>, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(User, r#"SELECT * FROM "user""#)
        .fetch_all(executor)
        .await?)
}

pub async fn find_by_username<'e, E>(executor: E, username: &str) -> Result<User, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        User,
        r#"SELECT * FROM "user" WHERE username = $1"#,
        username,
    )
    .fetch_one(executor)
    .await?)
}

pub async fn update_password<'e, E>(
    executor: E,
    user_id: &Uuid,
    new_password: &str,
) -> Result<User, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        User,
        r#"UPDATE "user" SET password = $2 WHERE id = $1 RETURNING *"#,
        user_id,
        new_password,
    )
    .fetch_one(executor)
    .await?)
}
