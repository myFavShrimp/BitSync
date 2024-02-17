use cynic::QueryBuilder;

use crate::api::{http::post_graphql_operation, GraphQlResult};

use super::super::schema::private as schema;

#[derive(Clone, Debug, cynic::QueryFragment)]
#[cynic(schema = "private", graphql_type = "User")]
pub struct User {
    username: String,
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
