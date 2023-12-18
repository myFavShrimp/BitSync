use crate::{
    database::user::User, handler::api::graphql::schema::public::Context,
    hash::verify_password_hash,
};

#[derive(thiserror::Error, Debug)]
#[error("Username or password wrong")]
pub enum LoginError {
    PasswordHash(#[from] argon2::password_hash::Error),
    DatabaseError(#[from] sqlx::Error),
}

pub async fn perform_login(
    ctx: &Context,
    username: String,
    password: String,
) -> Result<(), LoginError> {
    let user = User::find_by_username(&ctx.app_state.postgres_pool, &username).await?;

    verify_password_hash(&user.password, &password)?;

    Ok(())
}
