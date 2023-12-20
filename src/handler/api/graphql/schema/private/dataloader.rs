use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::user::User;

pub struct PostgresLoader {
    pool: PgPool,
}

impl PostgresLoader {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PostgresLoader {
    type Value = User;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        Ok(User::find_by_ids(&self.pool, keys)
            .await
            .map_err(Arc::new)?
            .into_iter()
            .map(|user| (user.id, user))
            .collect())
    }
}
