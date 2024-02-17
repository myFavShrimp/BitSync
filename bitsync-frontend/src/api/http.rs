use std::rc::Rc;

use leptos::SignalGetUntracked;

use crate::{api::API_PATH, global_storage::use_login_token};

use super::GraphQlResult;

static API_AUTH_HEADER: &str = "AUTHORIZATION";

#[derive(thiserror::Error, Debug, Clone)]
pub enum RequestError {
    #[error(transparent)]
    Gloo(#[from] Rc<gloo_net::Error>),
    #[error(transparent)]
    Json(#[from] Rc<serde_json::Error>),
}

async fn send_graphql_post_request<F, V>(
    operation: cynic::Operation<F, V>,
) -> Result<cynic::GraphQlResponse<F>, RequestError>
where
    F: for<'de> serde::Deserialize<'de>,
    V: serde::Serialize,
{
    let login_token = use_login_token().0.get_untracked();
    let json_operation = serde_json::to_value(operation).map_err(Rc::new)?;

    tracing::debug!("Sending GraphQL query `{:#?}`", json_operation);

    let mut request_builder = gloo_net::http::Request::post(API_PATH);
    if login_token != "" {
        request_builder =
            request_builder.header(API_AUTH_HEADER, &format!("Bearer {}", login_token));
    }

    let response = request_builder
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

#[derive(thiserror::Error, Debug, Clone)]
pub enum GraphQlError {
    #[error("A GraphQL api error occurred")]
    GraphQlApi(Vec<cynic::GraphQlError>),
    #[error("Received an invalid GraphQL response")]
    InvalidResponse,
}

trait GraphQlResponseIntoResult<F> {
    fn into_result(self) -> Result<F, GraphQlError>;
}

impl<F> GraphQlResponseIntoResult<F> for cynic::GraphQlResponse<F> {
    fn into_result(self) -> Result<F, GraphQlError> {
        if let Some(errors) = self.errors {
            Err(errors).map_err(GraphQlError::GraphQlApi)?
        } else if let Some(data) = self.data {
            Ok(data)
        } else {
            tracing::error!("GraphQL response is invalid");
            Err(GraphQlError::InvalidResponse)?
        }
    }
}

pub async fn post_graphql_operation<F, V>(operation: cynic::Operation<F, V>) -> GraphQlResult<F>
where
    F: for<'de> serde::Deserialize<'de>,
    V: serde::Serialize,
{
    Ok(send_graphql_post_request(operation).await?.into_result()?)
}
