use cynic::{GraphQlResponse, QueryBuilder};

use crate::api::API_PATH;

use super::super::schema::public as schema;

#[derive(cynic::QueryVariables, Clone)]
pub struct LoginQueryVariables {
    pub username: String,
    pub password: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "public",
    graphql_type = "Query",
    variables = "LoginQueryVariables"
)]
pub struct LoginQuery {
    #[arguments(username: $username, password: $password)]
    pub login: String,
}

pub async fn login(vars: LoginQueryVariables) -> Result<String, String> {
    let operation = LoginQuery::build(vars.clone());

    match gloo_net::http::Request::post(API_PATH)
        .json(&operation)
        .unwrap()
        .send()
        .await
    {
        Ok(val) => Ok(format!(
            "{:#?}",
            val.json::<GraphQlResponse<LoginQuery>>().await.unwrap()
        )),
        Err(e) => Err(e.to_string()),
    }
}
