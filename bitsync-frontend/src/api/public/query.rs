use cynic::QueryBuilder;

use crate::api::http::post_graphql_operation;

use super::super::{schema::public as schema, ApiError};

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

pub type LoginQueryResult = Result<LoginQuery, ApiError>;

pub async fn login(vars: LoginQueryVariables) -> LoginQueryResult {
    let operation = LoginQuery::build(vars.clone());

    post_graphql_operation(operation).await
}
