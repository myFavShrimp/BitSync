use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{FromRequest, Query, Request, State},
    middleware::from_fn_with_state,
    response::{IntoResponse, Response},
};
use axum_extra::{
    body::AsyncReadBody,
    extract::{Form, Multipart, multipart::Field},
    response::Attachment,
    routing::RouterExt,
};
use bitsync_core::use_case::{
    self,
    user_files::{
        create_directory::UserFileDirecoryCreationError,
        delete_user_file::UserFileDeletionError,
        download_user_file::UserFileDownloadError,
        move_user_file::UserFileMoveError,
        upload_user_file::{UserFileUploadError, upload_user_file},
    },
};
use bitsync_frontend::{
    Component, DIALOG_WRAPPER_SELECTOR, Render,
    pages::files::{
        FilesHomePageChangeResult,
        directory_creation::{
            DirectoryCreationDialog, DirectoryCreationDisplayError, DirectoryCreationForm,
        },
        file_move::FileMoveDialog,
        file_operations::{
            UserFileDeletionDisplayError, UserFileDownloadDisplayError, UserFileMoveDisplayError,
            UserFileUploadDisplayError,
        },
    },
};
use bitsync_hyperstim::{HyperStimCommand, HyperStimPatchMode};
use bitsync_routes::TypedPath;
use serde::Deserialize;

use crate::{
    AppState,
    auth::{AuthData, require_login_and_totp_setup_middleware},
    error_report::emit_error,
    handler::{RedirectHttp, RedirectHyperStim, user_error_toast_response},
};

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(
            Router::new()
                .typed_post(user_file_upload_handler)
                // .route_layer(axum::middleware::from_fn_with_state(
                //     state.clone(),
                //     crate::body_limit::dynamic_body_size_limit,
                // ))
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_login_and_totp_setup_middleware::<RedirectHyperStim>,
                ))
                .with_state(state.clone()),
        )
        .merge(
            Router::new()
                .typed_get(user_file_download_handler)
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_login_and_totp_setup_middleware::<RedirectHttp>,
                ))
                .with_state(state.clone()),
        )
        .merge(
            Router::new()
                .typed_get(user_file_delete_handler)
                .typed_post(user_file_move_handler)
                .typed_get(user_file_move_dialog_handler)
                .typed_post(user_file_directory_creation_handler)
                .typed_get(user_file_directory_creation_dialog_handler)
                .route_layer(from_fn_with_state(
                    state.clone(),
                    require_login_and_totp_setup_middleware::<RedirectHyperStim>,
                ))
                .with_state(state),
        )
}

struct UserFileMultipartField {
    pub field: Field,
    pub file_name: String,
}

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
            Ok(None) => {
                return Err(user_error_toast_response(
                    UserFileUploadDisplayError::NoFileProvided.message(),
                ));
            }
            Err(error) => return Err(error.body_text().into_response()),
        };

        let file_name = match multipart_field.file_name() {
            Some(file_name) => file_name.to_owned(),
            None => {
                return Err(user_error_toast_response(
                    UserFileUploadDisplayError::NoFileNameProvided.message(),
                ));
            }
        };

        Ok(Self {
            field: multipart_field,
            file_name,
        })
    }
}

async fn user_file_upload_handler(
    _: bitsync_routes::PostUserFileUpload,
    State(app_state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    query_parameters: Query<bitsync_routes::PostUserFileUploadQueryParameters>,
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
        Ok(result) => {
            let files_component = FilesHomePageChangeResult::from(result);

            Json(HyperStimCommand::HsPatchHtml {
                html: files_component.render(),
                patch_target: files_component.id_target(),
                patch_mode: HyperStimPatchMode::Outer,
            })
            .into_response()
        }
        Err(error) => {
            let display_error = match error {
                UserFileUploadError::StoragePath(..) => UserFileUploadDisplayError::InvalidPath,
                error => {
                    emit_error(error);
                    UserFileUploadDisplayError::InternalServerError
                }
            };

            user_error_toast_response(display_error.message())
        }
    }
}

async fn user_file_download_handler(
    _: bitsync_routes::GetUserFileDownload,
    State(app_state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    query_parameters: Query<bitsync_routes::GetUserFileDownloadQueryParameters>,
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
        Err(error) => {
            let display_error = match error {
                UserFileDownloadError::StoragePath(..) => UserFileDownloadDisplayError::InvalidPath,
                error => {
                    emit_error(error);
                    UserFileDownloadDisplayError::InternalServerError
                }
            };

            user_error_toast_response(display_error.message())
        }
    }
}

async fn user_file_delete_handler(
    _: bitsync_routes::GetUserFileDelete,
    State(app_state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    query_parameters: Query<bitsync_routes::GetUserFileDeleteQueryParameters>,
) -> impl IntoResponse {
    match use_case::user_files::delete_user_file::delete_user_file(
        &app_state.config.fs_storage_root_dir,
        &query_parameters.path,
        &auth_data.user,
    )
    .await
    {
        Ok(result) => {
            let files_component = FilesHomePageChangeResult::from(result);

            Json(HyperStimCommand::HsPatchHtml {
                html: files_component.render(),
                patch_target: files_component.id_target(),
                patch_mode: HyperStimPatchMode::Outer,
            })
            .into_response()
        }
        Err(error) => {
            let display_error = match error {
                UserFileDeletionError::StoragePath(..) => UserFileDeletionDisplayError::InvalidPath,
                error => {
                    emit_error(error);
                    UserFileDeletionDisplayError::InternalServerError
                }
            };

            user_error_toast_response(display_error.message())
        }
    }
}

#[derive(Deserialize)]
struct MoveItemFormData {
    pub destination_path: String,
}

async fn user_file_move_handler(
    _: bitsync_routes::PostUserFileMove,
    State(app_state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    query_parameters: Query<bitsync_routes::PostUserFileMoveQueryParameters>,
    Form(MoveItemFormData { destination_path }): Form<MoveItemFormData>,
) -> impl IntoResponse {
    match use_case::user_files::move_user_file::move_user_file(
        &app_state.config.fs_storage_root_dir,
        &query_parameters.path,
        &destination_path,
        &auth_data.user,
    )
    .await
    {
        Ok(result) => {
            let files_component = FilesHomePageChangeResult::from(result);
            let dialog_id =
                bitsync_frontend::pages::files::FilesHomePageElementId::FileMoveDialog.to_str();

            Json(vec![
                HyperStimCommand::HsPatchHtml {
                    html: files_component.render(),
                    patch_target: files_component.id_target(),
                    patch_mode: HyperStimPatchMode::Outer,
                },
                HyperStimCommand::HsExecute {
                    code: format!(
                        "closeClosestDialogAndRemoveElement(document.getElementById('{dialog_id}'))"
                    ),
                },
            ])
            .into_response()
        }
        Err(error) => {
            let display_error = match error {
                UserFileMoveError::StoragePath(..) => UserFileMoveDisplayError::InvalidPath,
                error => {
                    emit_error(error);
                    UserFileMoveDisplayError::InternalServerError
                }
            };

            user_error_toast_response(display_error.message())
        }
    }
}

#[derive(Deserialize)]
struct AddDirectoryFormData {
    pub directory_name: String,
}

async fn user_file_directory_creation_handler(
    _: bitsync_routes::PostUserFileDirectoryCreation,
    State(app_state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    query_parameters: Query<bitsync_routes::PostUserFileDirectoryCreationQueryParameters>,
    Form(AddDirectoryFormData { directory_name }): Form<AddDirectoryFormData>,
) -> impl IntoResponse {
    match use_case::user_files::create_directory::create_direcory(
        &app_state.config.fs_storage_root_dir,
        &query_parameters.path,
        &directory_name,
        &auth_data.user,
    )
    .await
    {
        Ok(result) => {
            let files_component = FilesHomePageChangeResult::from(result);
            let dialog_id =
                bitsync_frontend::pages::files::FilesHomePageElementId::DirectoryCreationDialog
                    .to_str();

            Json(vec![
                HyperStimCommand::HsPatchHtml {
                    html: files_component.render(),
                    patch_target: files_component.id_target(),
                    patch_mode: HyperStimPatchMode::Outer,
                },
                HyperStimCommand::HsExecute {
                    code: format!(
                        "closeClosestDialogAndRemoveElement(document.getElementById('{dialog_id}'))"
                    ),
                },
            ])
            .into_response()
        }
        Err(error) => {
            let display_error = match error {
                UserFileDirecoryCreationError::EmptyPath(..) => {
                    DirectoryCreationDisplayError::EmptyName
                }
                UserFileDirecoryCreationError::DirectoryNameContainsSeparator(..) => {
                    DirectoryCreationDisplayError::InvalidName
                }
                UserFileDirecoryCreationError::StoragePath(..) => {
                    DirectoryCreationDisplayError::InvalidPath
                }
                error => {
                    emit_error(error);
                    DirectoryCreationDisplayError::InternalServerError
                }
            };

            let directory_creation_url = bitsync_routes::PostUserFileDirectoryCreation
                .with_query_params(
                    bitsync_routes::PostUserFileDirectoryCreationQueryParameters {
                        path: query_parameters.path.clone(),
                    },
                )
                .to_string();

            let form = DirectoryCreationForm {
                action_url: directory_creation_url,
                error: Some(display_error),
            };

            Json(HyperStimCommand::HsPatchHtml {
                html: form.render(),
                patch_target: form.id_target(),
                patch_mode: HyperStimPatchMode::Outer,
            })
            .into_response()
        }
    }
}

async fn user_file_directory_creation_dialog_handler(
    _: bitsync_routes::GetUserFileDirectoryCreationDialog,
    query_parameters: Query<bitsync_routes::GetUserFileDirectoryCreationDialogQueryParameters>,
) -> impl IntoResponse {
    let action_url = bitsync_routes::PostUserFileDirectoryCreation
        .with_query_params(
            bitsync_routes::PostUserFileDirectoryCreationQueryParameters {
                path: query_parameters.path.clone(),
            },
        )
        .to_string();

    let dialog = DirectoryCreationDialog { action_url };

    Json(HyperStimCommand::HsPatchHtml {
        html: dialog.render(),
        patch_target: DIALOG_WRAPPER_SELECTOR.to_owned(),
        patch_mode: HyperStimPatchMode::Append,
    })
    .into_response()
}

async fn user_file_move_dialog_handler(
    _: bitsync_routes::GetUserFileMoveDialog,
    query_parameters: Query<bitsync_routes::GetUserFileMoveDialogQueryParameters>,
) -> impl IntoResponse {
    let action_url = bitsync_routes::PostUserFileMove
        .with_query_params(bitsync_routes::PostUserFileMoveQueryParameters {
            path: query_parameters.path.clone(),
        })
        .to_string();

    let dialog = FileMoveDialog {
        action_url,
        source_path: query_parameters.path.clone(),
    };

    Json(HyperStimCommand::HsPatchHtml {
        html: dialog.render(),
        patch_target: DIALOG_WRAPPER_SELECTOR.to_owned(),
        patch_mode: HyperStimPatchMode::Append,
    })
    .into_response()
}
