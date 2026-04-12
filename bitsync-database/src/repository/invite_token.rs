use sqlx::PgExecutor;
use uuid::Uuid;

use crate::entity::InviteToken;

use super::QueryError;

pub async fn create<'e, E>(connection: E) -> Result<InviteToken, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        InviteToken,
        r#"INSERT INTO "invite_token" (is_admin) VALUES (false) RETURNING *"#,
    )
    .fetch_one(connection)
    .await?)
}

pub async fn create_admin<'e, E>(connection: E) -> Result<InviteToken, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        InviteToken,
        r#"INSERT INTO "invite_token" (is_admin) VALUES (true) RETURNING *"#,
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

pub async fn find_all<'e, E>(executor: E) -> Result<Vec<InviteToken>, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(
        sqlx::query_as!(InviteToken, r#"SELECT * FROM "invite_token" ORDER BY id"#,)
            .fetch_all(executor)
            .await?,
    )
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
