use cynic::QueryBuilder;

use crate::api::post_graphql_operation;

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

    let graphql_response = post_graphql_operation(operation);

    return if let Some(errors) = graphql_response.errors {
        todo!("Not implemented")
    } else if let Some(data) = graphql_response.data {
        todo!("Not implemented")
    } else {
        todo!("Not implemented")
    };
}
