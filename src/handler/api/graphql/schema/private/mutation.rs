use std::io::Read;

use async_graphql::Upload;

use crate::database::user::User;

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
    ) -> async_graphql::Result<String> {
        let context = ctx.data::<Context>()?;

        let file_name = &files.get(0).unwrap().value(ctx).unwrap().filename;

        let mut current_dir = std::env::current_dir().unwrap();
        current_dir.push(context.current_user.id.to_string());
        current_dir.push(path);

        let mut fs_builder = opendal::services::Fs::default();
        fs_builder.root(current_dir.to_str().unwrap());

        let op = opendal::Operator::new(fs_builder)?.finish();

        let mut data = Vec::new();
        files
            .get(0)
            .unwrap()
            .value(ctx)
            .unwrap()
            .content
            .read_to_end(&mut data)
            .unwrap();

        op.write(&file_name, data).await.unwrap();

        Ok(String::new())
    }
}
