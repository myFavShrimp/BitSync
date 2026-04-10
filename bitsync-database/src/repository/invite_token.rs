use sqlx::PgExecutor;
use uuid::Uuid;

use crate::entity::InviteToken;

use super::QueryError;

pub async fn create<'e, E>(connection: E, is_admin: bool) -> Result<InviteToken, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        InviteToken,
        r#"INSERT INTO "invite_token" (is_admin) VALUES ($1) RETURNING *"#,
        is_admin,
    )
    .fetch_one(connection)
    .await?)
}

pub async fn find_by_id<'e, E>(executor: E, id: &Uuid) -> Result<Option<InviteToken>, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        InviteToken,
        r#"SELECT * FROM "invite_token" WHERE id = $1"#,
        id,
    )
    .fetch_optional(executor)
    .await?)
}

pub async fn find_admin_token<'e, E>(executor: E) -> Result<Option<InviteToken>, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        InviteToken,
        r#"SELECT * FROM "invite_token" WHERE is_admin = true LIMIT 1"#,
    )
    .fetch_optional(executor)
    .await?)
}

pub async fn delete_by_id<'e, E>(executor: E, id: &Uuid) -> Result<(), QueryError>
where
    E: PgExecutor<'e>,
{
    sqlx::query!(r#"DELETE FROM "invite_token" WHERE id = $1"#, id)
        .execute(executor)
        .await?;
    Ok(())
}
