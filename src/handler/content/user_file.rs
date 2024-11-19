use std::sync::Arc;

use axum::{
    extract::{FromRequest, Query, Request, State},
    http::StatusCode,
    middleware::from_fn_with_state,
    response::{Html, IntoResponse, Response},
    Router,
};
use axum_extra::{
    body::AsyncReadBody,
    extract::{
        multipart::{Field, MultipartError},
        Multipart,
    },
    response::Attachment,
    routing::RouterExt,
};
use bitsync_core::use_case::{self, user_files::upload_user_file::upload_user_file};

use crate::{
    auth::{require_login_middleware, AuthData},
    presentation::templates::FilesHomeFileStorageTableRowOobSwap,
    AppState,
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

struct UserFileMultipartField {
    pub field: Field,
    pub file_name: String,
}

#[async_trait::async_trait]
impl<S> FromRequest<S> for UserFileMultipartField
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut multipart_data = match Multipart::from_request(req, state).await {
            Ok(multipart_data) => multipart_data,
            Err(error) => return Err(error.into_response()),
        };

        let multipart_field = match multipart_data.next_field().await {
            Ok(Some(multipart_field)) => multipart_field,
            Err(_) | Ok(None) => todo!("error - no file"),
        };

        let file_name = match multipart_field.file_name() {
            Some(file_name) => file_name.to_owned(),
            None => todo!("error - no file"),
        };

        Ok(Self {
            field: multipart_field,
            file_name,
        })
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to read file stream")]
pub struct MappedMultipartError(#[from] MultipartError);

impl Into<std::io::Error> for MappedMultipartError {
    fn into(self) -> std::io::Error {
        todo!()
    }
}

async fn user_file_upload_handler(
    _: routes::PostUserFileUpload,
    State(app_state): State<Arc<AppState>>,
    auth_data: AuthData,
    query_parameters: Query<routes::PostUserFileUploadQueryParameters>,
    multipart_data: UserFileMultipartField,
) -> impl IntoResponse {
    match upload_user_file(
        &app_state.config.fs_storage_root_dir,
        &query_parameters.path,
        &auth_data.user,
        &multipart_data.file_name,
        multipart_data.field,
    )
    .await
    {
        Ok(result) => Html(FilesHomeFileStorageTableRowOobSwap::from(result).to_string()),
        Err(e) => todo!("{:#?}", e),
    }
}

async fn user_file_download_handler(
    _: routes::GetUserFileDownload,
    State(app_state): State<Arc<AppState>>,
    auth_data: AuthData,
    query_parameters: Query<routes::GetUserFileDownloadQueryParameters>,
) -> impl IntoResponse {
    match use_case::user_files::download_user_file::download_user_file(
        &app_state.config.fs_storage_root_dir,
        &query_parameters.path,
        &auth_data.user,
    )
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
    match use_case::user_files::delete_user_file::delete_user_file(
        &app_state.config.fs_storage_root_dir,
        &query_parameters.path,
        &auth_data.user,
    )
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => todo!(),
    }
}
