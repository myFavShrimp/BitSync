use std::sync::{atomic::Ordering, Arc};

use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::Limited;

use crate::AppState;

pub async fn dynamic_body_size_limit(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    let current_limit = state.current_file_upload_limit.load(Ordering::Relaxed);

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

        if content_length > current_limit {
            return StatusCode::PAYLOAD_TOO_LARGE.into_response();
        }
    }

    let limited_request = request.map(|body| Body::new(Limited::new(body, current_limit as usize)));

    next.run(limited_request).await
}
