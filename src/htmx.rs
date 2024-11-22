use std::sync::OnceLock;

use axum::response::{IntoResponse, Response};
use axum_extra::response::Html;

use crate::presentation::templates::error_modal::ErrorModal;

pub fn build_error_modal_oob_swap_response<E: std::error::Error>(error: E) -> Response {
    (Html(ErrorModal::from(error).to_string()),).into_response()
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
