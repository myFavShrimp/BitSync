use std::{convert::Infallible, sync::OnceLock};

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

static NONE_HX_RESWAP_HEADER: (&str, &str) = ("hx-reswap", "none");

pub fn build_error_modal_oob_swap_response<E: std::error::Error>(error: E) -> Response {
    (
        [NONE_HX_RESWAP_HEADER],
        Html(ErrorModal::from(error).to_string()),
    )
        .into_response()
}

static CONFIG_STRING: OnceLock<String> = OnceLock::new();

pub fn retrieve_config() -> &'static str {
    CONFIG_STRING.get_or_init(|| {
        serde_json::json!(
            {
                "defaultSwapStyle": "none",
                "responseHandling": [
                    {
                        "code": "[1234]..",
                        "swap": false,
                        "error": false,
                    },
                    {
                        "code": "5..",
                        "swap": false,
                        "error": true,
                    },
                ],
            }
        )
        .to_string()
    })
}
