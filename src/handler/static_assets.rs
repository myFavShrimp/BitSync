use axum::{extract::Path, response::IntoResponse, routing::get, Router};

use crate::handler::handler_404;

use super::routes;

pub(crate) async fn create_routes() -> Router {
    Router::new().route(&routes::Static::handler_route(), get(serve))
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

async fn serve(Path(file_path): Path<String>) -> impl IntoResponse {
    // let path = req.uri().path().trim_start_matches("/");

    tracing::debug!(file_path);

    ASSETS.iter().for_each(|a| {
        tracing::debug!(a.relative_path);
    });

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
