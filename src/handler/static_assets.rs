use std::sync::Arc;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::AppState;

use super::routes;

pub(crate) async fn create_routes(_state: Arc<AppState>) -> Router {
    Router::new().nest_service(&routes::Static::handler_route(), get(serve))
}

#[iftree::include_file_tree(
    "
        paths = '**'
        base_folder = 'static'
    "
)]
struct Asset {
    pub relative_path: &'static str,
    pub contents_str: &'static str,
}

async fn serve(req: Request<Body>) -> impl IntoResponse {
    let path = req.uri().path().trim_start_matches("/");

    tracing::debug!(path);

    ASSETS.iter().for_each(|a| {
        tracing::debug!(a.relative_path);
    });

    match ASSETS.iter().position(|asset| asset.relative_path == path) {
        None => StatusCode::NOT_FOUND.into_response(),
        Some(index) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            let content_type = headers::ContentType::from(mime);

            (
                axum_extra::TypedHeader(content_type),
                ASSETS[index].contents_str.to_owned(),
            )
                .into_response()
        }
    }
}
