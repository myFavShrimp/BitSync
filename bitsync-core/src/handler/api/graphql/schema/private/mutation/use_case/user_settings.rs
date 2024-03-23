use crate::{
    database::user::User, handler::api::graphql::schema::private::Context, hash::hash_password,
};

#[derive(thiserror::Error, Debug)]
#[error("An unexpected error occurred")]
pub enum UserSettingsError {
    PasswordHash(#[from] argon2::password_hash::Error),
    Database(#[from] sqlx::Error),
}

pub async fn update_password(ctx: &Context, new_password: &str) -> Result<User, UserSettingsError> {
    let hashed_password = hash_password(new_password)?;

    Ok(User::update_password(
        &ctx.app_state.postgres_pool,
        &ctx.current_user.id,
        &hashed_password,
    )
    .await?)
}
