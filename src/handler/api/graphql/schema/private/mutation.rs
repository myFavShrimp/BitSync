use async_graphql::Upload;

use crate::{
    database::user::User,
    handler::api::graphql::schema::FormattedStringError,
    storage::{DirItem, FileItem},
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

    async fn upload_user_files<'context>(
        &self,
        ctx: &async_graphql::Context<'context>,
        path: String,
        files: Vec<Upload>,
    ) -> async_graphql::Result<Vec<FileItem>> {
        Ok(use_case::user_files::upload_user_files(ctx, &path, files)
            .await
            .to_formatted_string_error()?)
    }

    async fn move_user_directory_item<'context>(
        &self,
        ctx: &async_graphql::Context<'context>,
        path: String,
        new_path: String,
    ) -> async_graphql::Result<DirItem> {
        Ok(
            use_case::user_files::move_user_directory_item(ctx, &path, &new_path)
                .await
                .to_formatted_string_error()?,
        )
    }

    async fn move_user_file_item<'context>(
        &self,
        ctx: &async_graphql::Context<'context>,
        path: String,
        new_path: String,
    ) -> async_graphql::Result<FileItem> {
        Ok(
            use_case::user_files::move_user_file_item(ctx, &path, &new_path)
                .await
                .to_formatted_string_error()?,
        )
    }

    async fn create_user_directory<'context>(
        &self,
        ctx: &async_graphql::Context<'context>,
        path: String,
    ) -> async_graphql::Result<DirItem> {
        Ok(use_case::user_files::create_user_directory(ctx, &path)
            .await
            .to_formatted_string_error()?)
    }
}
