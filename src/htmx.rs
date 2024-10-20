use std::convert::Infallible;

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap},
};

pub struct IsHxRequest(pub bool);

static HX_REQUEST_HEADER: &str = "hx-request";

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for IsHxRequest
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let headers = HeaderMap::from_request_parts(parts, state).await?;

        Ok(Self(headers.contains_key(HX_REQUEST_HEADER)))
    }
}
