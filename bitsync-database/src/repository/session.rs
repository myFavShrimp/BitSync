use sqlx::PgExecutor;
use uuid::Uuid;

use crate::entity::{Session, SessionBrowser, SessionPlatform};

use super::QueryError;

pub async fn create<'e, E>(
    executor: E,
    user_id: &Uuid,
    platform: &SessionPlatform,
    browser: &SessionBrowser,
) -> Result<Session, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        Session,
        r#"
            INSERT INTO "session" (user_id, platform, browser)
            VALUES ($1, $2, $3)
            RETURNING
                id,
                user_id,
                platform AS "platform: SessionPlatform",
                browser AS "browser: SessionBrowser",
                created_at,
                last_seen_at
        "#,
        user_id,
        platform as &SessionPlatform,
        browser as &SessionBrowser,
    )
    .fetch_one(executor)
    .await?)
}

pub async fn touch<'e, E>(executor: E, id: &Uuid) -> Result<Session, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        Session,
        r#"
            UPDATE "session"
            SET last_seen_at = now()
            WHERE id = $1
            RETURNING
                id,
                user_id,
                platform AS "platform: SessionPlatform",
                browser AS "browser: SessionBrowser",
                created_at,
                last_seen_at
        "#,
        id,
    )
    .fetch_one(executor)
    .await?)
}

pub async fn find_all_by_user_id<'e, E>(
    executor: E,
    user_id: &Uuid,
) -> Result<Vec<Session>, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        Session,
        r#"
            SELECT
                id,
                user_id,
                platform AS "platform: SessionPlatform",
                browser AS "browser: SessionBrowser",
                created_at,
                last_seen_at
            FROM "session"
            WHERE user_id = $1
            ORDER BY last_seen_at DESC
        "#,
        user_id,
    )
    .fetch_all(executor)
    .await?)
}

pub async fn delete_by_id<'e, E>(executor: E, id: &Uuid) -> Result<(), QueryError>
where
    E: PgExecutor<'e>,
{
    sqlx::query!(r#"DELETE FROM "session" WHERE id = $1"#, id)
        .execute(executor)
        .await?;

    Ok(())
}

pub async fn delete_all_by_user_id<'e, E>(executor: E, user_id: &Uuid) -> Result<(), QueryError>
where
    E: PgExecutor<'e>,
{
    sqlx::query!(r#"DELETE FROM "session" WHERE user_id = $1"#, user_id)
        .execute(executor)
        .await?;

    Ok(())
}

pub async fn delete_all_by_user_id_except<'e, E>(
    executor: E,
    user_id: &Uuid,
    except_session_id: &Uuid,
) -> Result<(), QueryError>
where
    E: PgExecutor<'e>,
{
    sqlx::query!(
        r#"DELETE FROM "session" WHERE user_id = $1 AND id != $2"#,
        user_id,
        except_session_id,
    )
    .execute(executor)
    .await?;

    Ok(())
}
