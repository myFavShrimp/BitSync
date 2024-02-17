use cynic::QueryBuilder;

use crate::api::{http::post_graphql_operation, GraphQlResult};

use super::super::schema::public as schema;

#[derive(cynic::QueryVariables, Clone)]
pub struct LoginQueryVariables {
    pub username: String,
    pub password: String,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(
    schema = "public",
    graphql_type = "Query",
    variables = "LoginQueryVariables"
)]
pub struct LoginQuery {
    #[arguments(username: $username, password: $password)]
    pub login: String,
}

pub async fn login(vars: LoginQueryVariables) -> GraphQlResult<LoginQuery> {
    let operation = LoginQuery::build(vars.clone());

    post_graphql_operation(operation).await
}
