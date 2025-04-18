use sqlx::PgExecutor;
use uuid::Uuid;

use crate::entity::User;

use super::QueryError;

pub async fn create<'e, E>(
    connection: E,
    username: &str,
    password: &str,
    totp_secret: &[u8],
) -> Result<User, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        User,
        r#"INSERT INTO "user" (username, password, totp_secret) VALUES ($1, $2, $3) RETURNING *"#,
        username,
        password,
        totp_secret,
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

pub async fn set_totp_setup_state<'e, E>(
    executor: E,
    user_id: &Uuid,
    is_totp_setup: bool,
) -> Result<User, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        User,
        r#"UPDATE "user" SET is_totp_set_up = $2 WHERE id = $1 RETURNING *"#,
        user_id,
        is_totp_setup,
    )
    .fetch_one(executor)
    .await?)
}
