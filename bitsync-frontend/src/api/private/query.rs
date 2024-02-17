use cynic::QueryBuilder;
use uuid::Uuid;

use crate::api::{http::post_graphql_operation, GraphQlResult};

use super::super::schema::private as schema;

cynic::impl_scalar!(Uuid, schema::UUID);

#[derive(Clone, Debug, cynic::QueryFragment)]
#[cynic(schema = "private", graphql_type = "User")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub is_admin: bool,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(schema = "private", graphql_type = "Query")]
pub struct MeQuery {
    pub me: User,
}

pub async fn me() -> GraphQlResult<MeQuery> {
    let operation = MeQuery::build(());

    post_graphql_operation(operation).await
}
