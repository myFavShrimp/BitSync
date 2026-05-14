use axum::{
    Json,
    body::Body,
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use bitsync_frontend::{BODY_SELECTOR_TARGET, Render, error_modal::ErrorModal};
use bitsync_hyperstim::{HyperStimCommand, HyperStimPatchMode};
use http_body_util::Limited;

/// 2mb
const REGULAR_BODY_SIZE_LIMIT: u64 = 2 * 1024 * 1024;

#[tracing::instrument(skip(request, next))]
pub async fn request_body_size_limit(request: Request, next: Next) -> Response {
    if let Some(content_length_header_value) = request.headers().get(header::CONTENT_LENGTH) {
        let content_length = match content_length_header_value.to_str() {
            Ok(content_length_str) => match content_length_str.parse::<u64>() {
                Ok(content_length) => content_length,
                Err(_) => {
                    return StatusCode::BAD_REQUEST.into_response();
                }
            },
            Err(_) => {
                return StatusCode::BAD_REQUEST.into_response();
            }
        };

        if content_length > REGULAR_BODY_SIZE_LIMIT {
            // TODO: this is broken. Firefox produces an NS_ERROR_NET_RESET error
            // because the connection is closed too early.
            // See: - https://github.com/tokio-rs/axum/discussions/2445
            //      - https://github.com/tokio-rs/axum/issues/2850
            // Possible fix?: https://github.com/hyperium/http-body/blob/master/http-body-util/src/limited.rs#L32
            return (
                StatusCode::PAYLOAD_TOO_LARGE,
                Json(HyperStimCommand::HsPatchHtml {
                    html: ErrorModal::with_message(format!(
                        "The upload is too large. The maximum allowed size is {} bytes.",
                        REGULAR_BODY_SIZE_LIMIT
                    ))
                    .render(),
                    patch_target: BODY_SELECTOR_TARGET.to_owned(),
                    patch_mode: HyperStimPatchMode::Append,
                }),
            )
                .into_response();
        }
    }

    let limited_request =
        request.map(|body| Body::new(Limited::new(body, REGULAR_BODY_SIZE_LIMIT as usize)));

    next.run(limited_request).await
}
