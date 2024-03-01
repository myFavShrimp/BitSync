use std::rc::Rc;

#[derive(thiserror::Error, Debug, Clone)]
pub enum ResponseError {
    #[error("A GraphQL api error occurred")]
    GraphQlApi(Vec<cynic::GraphQlError>),
    #[error("Received an invalid GraphQL response")]
    InvalidResponse,
    #[error("Couldn't deserialize GraphQL response")]
    Deserialization(#[from] Rc<serde_json::Error>),
    #[error("An error occurred whil receiving the response")]
    Net(#[from] Rc<gloo_net::Error>),
}

pub async fn handle_graphql_response<F>(
    response: gloo_net::http::Response,
) -> Result<F, ResponseError>
where
    F: for<'de> serde::Deserialize<'de>,
{
    let response_data =
        serde_json::from_str::<cynic::GraphQlResponse<F>>(&response.text().await.map_err(Rc::new)?)
            .map_err(Rc::new)?;

    if let Some(errors) = response_data.errors {
        Err(ResponseError::GraphQlApi(errors))?
    } else if let Some(data) = response_data.data {
        Ok(data)
    } else {
        tracing::error!("GraphQL response is invalid");
        Err(ResponseError::InvalidResponse)?
    }
}
