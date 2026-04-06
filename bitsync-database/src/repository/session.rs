use sqlx::PgExecutor;
use uuid::Uuid;

use crate::entity::{Session, SessionPlatform};

use super::QueryError;

pub async fn create<'e, E>(
    executor: E,
    user_id: &Uuid,
    platform: &SessionPlatform,
) -> Result<Session, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        Session,
        r#"INSERT INTO "session" (user_id, platform) VALUES ($1, $2) RETURNING id, user_id, platform AS "platform: SessionPlatform", created_at, last_seen_at"#,
        user_id,
        platform as &SessionPlatform,
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
        r#"UPDATE "session" SET last_seen_at = now() WHERE id = $1 RETURNING id, user_id, platform AS "platform: SessionPlatform", created_at, last_seen_at"#,
        id,
    )
    .fetch_one(executor)
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
