use uuid::Uuid;

use crate::database::user::User;

#[async_graphql::Object]
impl User {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn username(&self) -> &str {
        &self.username
    }

    async fn is_admin(&self) -> bool {
        self.is_admin
    }

    async fn color_palette(&self) -> Option<String> {
        self.color_palette.clone()
    }
}
