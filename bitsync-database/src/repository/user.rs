use sqlx::PgExecutor;
use uuid::Uuid;

use crate::entity::User;

use super::QueryError;

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

pub async fn find_by_username<'e, E>(
    executor: E,
    username: &str,
) -> Result<Option<User>, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        User,
        r#"SELECT * FROM "user" WHERE username = $1"#,
        username,
    )
    .fetch_optional(executor)
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

pub async fn create_with_admin<'e, E>(
    connection: E,
    username: &str,
    password: &str,
    dangling_totp_secret: &[u8],
    is_admin: bool,
) -> Result<User, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        User,
        r#"INSERT INTO "user" (username, password, dangling_totp_secret, is_admin) VALUES ($1, $2, $3, $4) RETURNING *"#,
        username,
        password,
        dangling_totp_secret,
        is_admin,
    )
    .fetch_one(connection)
    .await?)
}

pub async fn admin_exists<'e, E>(executor: E) -> Result<bool, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM "user" WHERE is_admin = true) as "exists!: bool""#,
    )
    .fetch_one(executor)
    .await?)
}

pub async fn activate_dangling_totp_secret<'e, E>(
    executor: E,
    user_id: &Uuid,
) -> Result<User, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        User,
        r#"UPDATE "user" SET active_totp_secret = dangling_totp_secret, dangling_totp_secret = NULL WHERE id = $1 RETURNING *"#,
        user_id,
    )
    .fetch_one(executor)
    .await?)
}

pub async fn set_admin<'e, E>(
    executor: E,
    user_id: &Uuid,
    is_admin: bool,
) -> Result<User, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        User,
        r#"UPDATE "user" SET is_admin = $2 WHERE id = $1 RETURNING *"#,
        user_id,
        is_admin,
    )
    .fetch_one(executor)
    .await?)
}

pub async fn clear_totp_secret<'e, E>(executor: E, user_id: &Uuid) -> Result<User, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        User,
        r#"UPDATE "user" SET active_totp_secret = NULL WHERE id = $1 RETURNING *"#,
        user_id,
    )
    .fetch_one(executor)
    .await?)
}

pub async fn set_suspended<'e, E>(
    executor: E,
    user_id: &Uuid,
    is_suspended: bool,
) -> Result<User, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        User,
        r#"UPDATE "user" SET is_suspended = $2 WHERE id = $1 RETURNING *"#,
        user_id,
        is_suspended,
    )
    .fetch_one(executor)
    .await?)
}

pub async fn delete<'e, E>(executor: E, user_id: &Uuid) -> Result<(), QueryError>
where
    E: PgExecutor<'e>,
{
    sqlx::query!(r#"DELETE FROM "user" WHERE id = $1"#, user_id)
        .execute(executor)
        .await?;

    Ok(())
}

pub async fn set_dangling_totp_secret<'e, E>(
    executor: E,
    user_id: &Uuid,
    dangling_totp_secret: &[u8],
) -> Result<User, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        User,
        r#"UPDATE "user" SET dangling_totp_secret = $2 WHERE id = $1 RETURNING *"#,
        user_id,
        dangling_totp_secret,
    )
    .fetch_one(executor)
    .await?)
}
