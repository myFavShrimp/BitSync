use sqlx::PgExecutor;
use uuid::Uuid;

use crate::entity::TotpRecoveryCode;

use super::QueryError;

pub async fn create<'e, E>(
    connection: E,
    user_id: Uuid,
    totp_secrets: &[String; 4],
) -> Result<Vec<TotpRecoveryCode>, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        TotpRecoveryCode,
        r#"
            INSERT INTO "totp_recovery_code" (user_id, code)
            VALUES
                ($1, $2),
                ($1, $3),
                ($1, $4),
                ($1, $5)
            RETURNING *
        "#,
        user_id,
        totp_secrets[0],
        totp_secrets[1],
        totp_secrets[2],
        totp_secrets[3],
    )
    .fetch_all(connection)
    .await?)
}

pub async fn find_by_user_id<'e, E>(
    executor: E,
    user_id: Uuid,
) -> Result<Vec<TotpRecoveryCode>, QueryError>
where
    E: PgExecutor<'e>,
{
    Ok(sqlx::query_as!(
        TotpRecoveryCode,
        r#"SELECT * FROM "totp_recovery_code" WHERE user_id = $1"#,
        user_id,
    )
    .fetch_all(executor)
    .await?)
}

pub async fn delete_all_for_user<'e, E>(executor: E, user_id: &Uuid) -> Result<(), QueryError>
where
    E: PgExecutor<'e>,
{
    sqlx::query!(
        r#"DELETE FROM "totp_recovery_code" WHERE user_id = $1"#,
        user_id,
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn delete<'e, E>(executor: E, user_id: &Uuid, code_hash: &str) -> Result<(), QueryError>
where
    E: PgExecutor<'e>,
{
    sqlx::query!(
        r#"DELETE FROM "totp_recovery_code" WHERE user_id = $1 AND code = $2"#,
        user_id,
        code_hash,
    )
    .execute(executor)
    .await?;

    Ok(())
}
