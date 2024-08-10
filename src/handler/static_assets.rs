use axum::{response::IntoResponse, Router};
use axum_extra::routing::RouterExt;

use crate::handler::handler_404;

use super::routes;

pub(crate) async fn create_routes() -> Router {
    Router::new().typed_get(serve)
}

#[iftree::include_file_tree(
    "
        paths = '**'
        base_folder = 'static'
    "
)]
struct Asset {
    pub relative_path: &'static str,
    pub contents_bytes: &'static [u8],
}

async fn serve(routes::Static { file_path }: routes::Static) -> impl IntoResponse {
    match ASSETS
        .iter()
        .position(|asset| asset.relative_path == file_path)
    {
        None => handler_404().await.into_response(),
        Some(index) => {
            let mime = mime_guess::from_path(file_path).first_or_octet_stream();
            let content_type = headers::ContentType::from(mime);

            (
                axum_extra::TypedHeader(content_type),
                ASSETS[index].contents_bytes,
            )
                .into_response()
        }
    }
}
