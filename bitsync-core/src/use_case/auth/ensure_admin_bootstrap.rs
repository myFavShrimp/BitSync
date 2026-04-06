use bitsync_database::{
    database::{ConnectionAcquisitionError, Database},
    entity::InviteToken,
    repository,
};

#[derive(thiserror::Error, Debug)]
#[error("admin bootstrap failed")]
pub enum EnsureAdminBootstrapError {
    DatabaseQuery(#[from] repository::QueryError),
    DatabaseConnectionAcquisition(#[from] ConnectionAcquisitionError),
}

pub enum AdminBootstrapStatus {
    Ready,
    RegistrationRequired(InviteToken),
}

pub async fn ensure_admin_bootstrap(
    database: &Database,
) -> Result<AdminBootstrapStatus, EnsureAdminBootstrapError> {
    let mut connection = database.acquire_connection().await?;

    let admin_exists = repository::user::admin_exists(&mut *connection).await?;

    if admin_exists {
        return Ok(AdminBootstrapStatus::Ready);
    }

    let admin_token = repository::invite_token::find_admin_token(&mut *connection).await?;

    let token = match admin_token {
        Some(existing) => existing,
        None => repository::invite_token::create(&mut *connection, true).await?,
    };

    Ok(AdminBootstrapStatus::RegistrationRequired(token))
}
