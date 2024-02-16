use std::rc::Rc;

static API_PATH: &str = "http://localhost:8080/api/graphql";

pub mod public;

mod schema {
    #[cynic::schema("public")]
    pub mod public {}
    #[cynic::schema("private")]
    pub mod private {}
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum RequestError {
    #[error(transparent)]
    Gloo(#[from] Rc<gloo_net::Error>),
    #[error(transparent)]
    Json(#[from] Rc<serde_json::Error>),
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ApiError {
    #[error(transparent)]
    Request(#[from] RequestError),
    #[error(transparent)]
    GraphQl(#[from] cynic::GraphQlError),
}

async fn post_graphql_operation<F, V>(
    operation: cynic::Operation<F, V>,
) -> Result<cynic::GraphQlResponse<F>, RequestError>
where
    F: for<'de> serde::Deserialize<'de>,
    V: serde::Serialize,
{
    let response = gloo_net::http::Request::post(API_PATH)
        .json(&operation)
        .map_err(Rc::new)?
        .send()
        .await
        .map_err(Rc::new)?;

    Ok(response
        .json::<cynic::GraphQlResponse<F>>()
        .await
        .map_err(Rc::new)?)
}
