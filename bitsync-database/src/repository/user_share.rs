use sqlx::PgExecutor;
use uuid::Uuid;

use crate::entity::UserShare;

use super::QueryError;

pub async fn create<'e, E>(
    connection: E,
    user_id: &Uuid,
    item_path: &str,
) -> Result<UserShare, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        UserShare,
        r#"INSERT INTO "user_share" (user_id, item_path) VALUES ($1, $2) RETURNING *"#,
        user_id,
        item_path,
    )
    .fetch_one(connection)
    .await?)
}

pub async fn find_all_by_user_id_and_item_path<'e, E>(
    executor: E,
    user_id: &Uuid,
    item_path: &str,
) -> Result<Vec<UserShare>, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        UserShare,
        r#"SELECT * FROM "user_share" WHERE user_id = $1 AND item_path = $2 ORDER BY id"#,
        user_id,
        item_path,
    )
    .fetch_all(executor)
    .await?)
}

pub async fn delete_by_id<'e, E>(executor: E, id: &Uuid, user_id: &Uuid) -> Result<(), QueryError>
where
    E: PgExecutor<'e>,
{
    sqlx::query!(
        r#"DELETE FROM "user_share" WHERE id = $1 AND user_id = $2"#,
        id,
        user_id,
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn find_distinct_item_paths_by_user_id<'e, E>(
    executor: E,
    user_id: &Uuid,
) -> Result<Vec<String>, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_scalar!(
        r#"SELECT DISTINCT item_path FROM "user_share" WHERE user_id = $1 ORDER BY item_path"#,
        user_id,
    )
    .fetch_all(executor)
    .await?)
}

pub async fn delete_all_by_user_id_and_item_path<'e, E>(
    executor: E,
    user_id: &Uuid,
    item_path: &str,
) -> Result<(), QueryError>
where
    E: PgExecutor<'e>,
{
    sqlx::query!(
        r#"DELETE FROM "user_share" WHERE user_id = $1 AND item_path = $2"#,
        user_id,
        item_path,
    )
    .execute(executor)
    .await?;

    Ok(())
}
