use std::convert::Infallible;

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap},
    response::{IntoResponse, Response},
};
use axum_extra::response::Html;

use crate::presentation::templates::error_modal::ErrorModal;

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

pub fn build_error_modal_oob_swap_response<E: std::error::Error>(error: E) -> Response {
    (
        [("hx-reswap", "none")],
        Html(ErrorModal::from(error).to_string()),
    )
        .into_response()
}
