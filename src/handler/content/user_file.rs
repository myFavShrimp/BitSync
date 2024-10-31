use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    middleware::from_fn_with_state,
    response::IntoResponse,
    Router,
};
use axum_extra::{
    body::AsyncReadBody, extract::Multipart, response::Attachment, routing::RouterExt,
};

use crate::{
    auth::{require_login_middleware, AuthData},
    use_case, AppState,
};

use super::routes;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_post(user_file_upload_handler)
        .typed_get(user_file_download_handler)
        .typed_get(user_file_delete_handler)
        .route_layer(from_fn_with_state(state.clone(), require_login_middleware))
        .with_state(state)
}

async fn user_file_upload_handler(
    _: routes::PostUserFileUpload,
    State(app_state): State<Arc<AppState>>,
    auth_data: AuthData,
    query_parameters: Query<routes::PostUserFileUploadQueryParameters>,
    multipart_data: Multipart,
) -> impl IntoResponse {
    match use_case::user_files::user_file_upload(
        &app_state,
        &auth_data,
        &query_parameters.path,
        multipart_data,
    )
    .await
    {
        Ok(result) => {
            let content_type = headers::ContentType::from(result.mime);

            let stream_body = AsyncReadBody::new(result.file);

            let attachment = Attachment::new(stream_body).filename(result.path.file_name());

            (axum_extra::TypedHeader(content_type), attachment).into_response()
        }
        Err(e) => todo!("{:#?}", e),
    }
}

async fn user_file_download_handler(
    _: routes::GetUserFileDownload,
    State(app_state): State<Arc<AppState>>,
    auth_data: AuthData,
    query_parameters: Query<routes::GetUserFileDownloadQueryParameters>,
) -> impl IntoResponse {
    match use_case::user_files::user_file_download(&app_state, &auth_data, &query_parameters.path)
        .await
    {
        Ok(result) => {
            let content_type = headers::ContentType::from(result.mime);

            let stream_body = AsyncReadBody::new(result.file);

            let attachment = Attachment::new(stream_body).filename(result.path.file_name());

            (axum_extra::TypedHeader(content_type), attachment).into_response()
        }
        Err(_) => todo!(),
    }
}

async fn user_file_delete_handler(
    _: routes::GetUserFileDelete,
    State(app_state): State<Arc<AppState>>,
    auth_data: AuthData,
    query_parameters: Query<routes::GetUserFileDeleteQueryParameters>,
) -> impl IntoResponse {
    match use_case::user_files::user_file_delete(&app_state, &auth_data, &query_parameters.path)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => todo!(),
    }
}
