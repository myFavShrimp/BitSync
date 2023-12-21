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
        mut files: Vec<Upload>,
    ) -> async_graphql::Result<String> {
        let context = ctx.data::<Context>()?;

        let mut current_dir = std::env::current_dir().unwrap();
        current_dir.push(context.current_user.id.to_string());
        current_dir.push(path);

        let mut fs_builder = opendal::services::Fs::default();
        fs_builder.root(current_dir.to_str().unwrap());

        let op = opendal::Operator::new(fs_builder)?.finish();

        let files: Result<Vec<async_graphql::UploadValue>, std::io::Error> =
            files.iter_mut().map(|file| file.value(ctx)).collect();

        for file in files.unwrap() {
            let file_name = &file.filename;
            let mut file_content = file.content;

            let mut data = Vec::new();
            file_content.read_to_end(&mut data).unwrap();

            op.write(&file_name, data).await.unwrap();
        }

        Ok(String::new())
    }
}
