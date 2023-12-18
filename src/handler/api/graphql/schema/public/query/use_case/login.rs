use crate::{database::user::User, handler::api::graphql::schema::public::Context};

#[derive(thiserror::Error, Debug)]
#[error("Username or password wrong")]
pub struct UsernameOrPasswordWrongError;

pub async fn perform_login(
    ctx: &Context,
    username: String,
    password: String,
) -> async_graphql::Result<()> {
    let user = User::find_by_username(&ctx.app_state.postgres_pool, &username)
        .await
        .map_err(|_| UsernameOrPasswordWrongError)?;

    if user.password == password {
        Ok(())
    } else {
        Err(UsernameOrPasswordWrongError)?
    }
}
