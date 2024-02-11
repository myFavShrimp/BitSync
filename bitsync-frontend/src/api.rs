static API_PATH: &str = "http://localhost:8080/api/graphql";

mod schema {
    #[cynic::schema("public")]
    pub mod public {}
    #[cynic::schema("private")]
    pub mod private {}
}

pub mod query {
    use cynic::{GraphQlResponse, QueryBuilder};

    use crate::api::API_PATH;

    use super::schema::public as schema;

    #[derive(cynic::QueryVariables)]
    pub struct LoginQueryVariables {
        username: String,
        password: String,
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

    pub async fn login() -> Result<String, String> {
        let x = LoginQuery::build(LoginQueryVariables {
            username: String::from("test"),
            password: String::from("test"),
        });

        tracing::debug!("{}", x.query);

        match gloo_net::http::Request::post(API_PATH)
            .json(&x)
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
}
