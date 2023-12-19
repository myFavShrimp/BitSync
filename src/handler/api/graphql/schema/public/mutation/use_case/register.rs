use crate::{
    database::user::User, handler::api::graphql::schema::public::Context, hash::hash_password,
};

#[derive(thiserror::Error, Debug)]
#[error("An unexpected error occurred")]
pub enum RegistrationError {
    PasswordHash(#[from] argon2::password_hash::Error),
    Database(#[from] sqlx::Error),
    #[error("The username already exists")]
    UserExists,
}

pub async fn perform_registration(
    ctx: &Context,
    username: String,
    password: String,
) -> Result<User, RegistrationError> {
    if let Ok(_user) = User::find_by_username(&ctx.app_state.postgres_pool, &username).await {
        return Err(RegistrationError::UserExists);
    }

    let hashed_password = hash_password(&password)?;

    Ok(User::create(&ctx.app_state.postgres_pool, &username, &hashed_password).await?)
}
