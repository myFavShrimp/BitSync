use async_graphql::Upload;

use crate::{
    database::user::User, dto::DirectoryEntry, handler::api::graphql::schema::FormattedStringError,
};

use super::Context;

mod use_case;

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn update_password<'context>(
        &self,
        context: &async_graphql::Context<'context>,
        new_password: String,
    ) -> async_graphql::Result<User> {
        let context = context.data::<Context>()?;

        Ok(use_case::user_settings::update_password(context, &new_password).await?)
    }

    async fn upload_user_file<'context>(
        &self,
        ctx: &async_graphql::Context<'context>,
        path: String,
        files: Vec<Upload>,
    ) -> async_graphql::Result<Vec<DirectoryEntry>> {
        Ok(use_case::user_files::upload_user_file(ctx, &path, files)
            .await
            .to_formatted_string_error()?)
    }
}
