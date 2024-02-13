use std::rc::Rc;

use cynic::{GraphQlResponse, QueryBuilder};

use crate::api::{ApiError, API_PATH};

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

pub async fn login(vars: LoginQueryVariables) -> Result<LoginQuery, ApiError> {
    let operation = LoginQuery::build(vars.clone());

    let response = gloo_net::http::Request::post(API_PATH)
        .json(&operation)
        .map_err(Rc::new)?
        .send()
        .await
        .map_err(Rc::new)?;

    Ok(response
        .json::<GraphQlResponse<LoginQuery>>()
        .await
        .map_err(Rc::new)?
        .data
        .unwrap())
}
