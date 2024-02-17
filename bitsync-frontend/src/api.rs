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
pub enum GraphQlError {
    #[error("A GraphQL api error occurred")]
    GraphQlApi(Vec<cynic::GraphQlError>),
    #[error("Received an invalid GraphQL response")]
    InvalidResponse,
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ApiError {
    #[error(transparent)]
    Request(#[from] RequestError),
    #[error(transparent)]
    GraphQl(#[from] GraphQlError),
}

async fn post_graphql_operation<F, V>(
    operation: cynic::Operation<F, V>,
) -> Result<cynic::GraphQlResponse<F>, RequestError>
where
    F: for<'de> serde::Deserialize<'de>,
    V: serde::Serialize,
{
    let json_operation = serde_json::to_value(operation).map_err(Rc::new)?;

    tracing::debug!("Sending GraphQL query `{:#?}`", json_operation);

    let response = gloo_net::http::Request::post(API_PATH)
        .json(&json_operation)
        .map_err(Rc::new)?
        .send()
        .await
        .map_err(Rc::new)?;

    tracing::debug!("Got GraphQL response `{:#?}`", response);

    Ok(response
        .json::<cynic::GraphQlResponse<F>>()
        .await
        .map_err(Rc::new)?)
}

trait GraphQlResponseIntoResult<F> {
    fn into_result(self) -> Result<F, ApiError>;
}

impl<F> GraphQlResponseIntoResult<F> for cynic::GraphQlResponse<F> {
    fn into_result(self) -> Result<F, ApiError> {
        if let Some(errors) = self.errors {
            Err(errors).map_err(GraphQlError::GraphQlApi)?
        } else if let Some(data) = self.data {
            Ok(data)
        } else {
            tracing::error!("Received invalid GraphQL response");
            Err(GraphQlError::InvalidResponse)?
        }
    }
}
