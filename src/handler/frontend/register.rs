use std::sync::Arc;

use axum::{
    Extension, Router,
    extract::State,
    middleware::from_fn_with_state,
    response::{Html, IntoResponse},
};
use axum_extra::{
    extract::{CookieJar, Form},
    routing::RouterExt,
};
use axum_htmx::HxRequest;
use bitsync_core::use_case::auth::{
    registration::{RegistrationError, perform_registration},
    retrieve_totp_setup_data::{RetrieveTotpSetupDataError, retrieve_totp_setup_data},
    setup_totp::{TotpSetupError, setup_totp},
};
use bitsync_frontend::{
    Render,
    pages::register::{RegisterForm, RegisterPage, TotpRecoveryCodesPrompt, TotpSetupForm},
};
use serde::Deserialize;

use crate::{
    auth::{
        AuthData, jwt_cookie, require_login_and_no_totp_setup_middleware, require_logout_middleware,
    },
    handler::redirect_response,
};

use crate::AppState;

use super::UNEXPECTED_ERROR_MESSAGE;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    let totp_setup_router = Router::new()
        .typed_get(register_totp_setup_page_handler)
        .typed_post(register_totp_setup_submit_handler)
        .route_layer(from_fn_with_state(
            state.clone(),
            require_login_and_no_totp_setup_middleware,
        ))
        .with_state(state.clone());

    Router::new()
        .typed_get(register_page_handler)
        .typed_post(register_action_handler)
        .route_layer(from_fn_with_state(state.clone(), require_logout_middleware))
        .merge(totp_setup_router)
        .with_state(state)
}

async fn register_page_handler(_: bitsync_routes::GetRegisterPage) -> impl IntoResponse {
    Html(RegisterPage::default().render())
}

#[derive(Deserialize, Clone, Debug)]
struct RegisterActionFormData {
    username: String,
    password: String,
}

async fn register_action_handler(
    _: bitsync_routes::PostRegisterAction,
    HxRequest(is_hx_request): HxRequest,
    State(state): State<Arc<AppState>>,
    cookie_jar: CookieJar,
    Form(registration_data): Form<RegisterActionFormData>,
) -> impl IntoResponse {
    match perform_registration(
        &state.database,
        &state.config.fs_storage_root_dir,
        &registration_data.username,
        &registration_data.password,
        state.config.auth.jwt_expiration_seconds,
        &state.config.auth.jwt_secret,
    )
    .await
    {
        Ok(result) => {
            let cookie_jar = cookie_jar.add(jwt_cookie(&result.jwt));

            (
                cookie_jar,
                redirect_response(
                    is_hx_request,
                    &bitsync_routes::GetRegisterTotpSetupPage.to_string(),
                ),
            )
                .into_response()
        }
        Err(error) => Html(
            RegisterForm {
                username: Some(registration_data.username),
                error_message: Some(match error {
                    RegistrationError::EnsureUserStorageExists(..)
                    | RegistrationError::Jwt(..)
                    | RegistrationError::PasswordHash(..)
                    | RegistrationError::DatabaseQuery(..)
                    | RegistrationError::DatabaseTransaction(..)
                    | RegistrationError::TransactionBegin(..) => {
                        String::from(UNEXPECTED_ERROR_MESSAGE)
                    }
                    RegistrationError::UserExists(..) => {
                        String::from("The username is already taken.")
                    }
                }),
            }
            .render(),
        )
        .into_response(),
    }
}

async fn register_totp_setup_page_handler(
    _: bitsync_routes::GetRegisterTotpSetupPage,
    Extension(auth_data): Extension<AuthData>,
    HxRequest(is_hx_request): HxRequest,
) -> impl IntoResponse {
    match retrieve_totp_setup_data(&auth_data.user).await {
        Ok(totp_setup_data) => Html(
            RegisterPage::TotpSetup(TotpSetupForm {
                totp_secret_image_base64_img_src: totp_setup_data.secret_base64_qr_code,
                totp_secret: totp_setup_data.secret_base32,
                error_message: None,
            })
            .render(),
        )
        .into_response(),
        Err(error) => {
            Html(
                RegisterPage::TotpSetup(TotpSetupForm {
                    // TODO: make optional
                    totp_secret_image_base64_img_src: String::new(),
                    totp_secret: String::new(),
                    // TODO: display load errors at the top?
                    error_message: Some(match error {
                        RetrieveTotpSetupDataError::TotpAlreadySetUp(..) => {
                            return redirect_response(
                                is_hx_request,
                                &bitsync_routes::GetFilesHomePage.to_string(),
                            );
                        }
                        RetrieveTotpSetupDataError::TotpCreation(..)
                        | RetrieveTotpSetupDataError::TotpSecretBase64QrCode(..) => {
                            String::from(UNEXPECTED_ERROR_MESSAGE)
                        }
                    }),
                })
                .render(),
            )
            .into_response()
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
struct TotpSetupFormData {
    totp: String,
}

async fn register_totp_setup_submit_handler(
    _: bitsync_routes::PostRegisterTotpSetupAction,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    HxRequest(is_hx_request): HxRequest,
    cookie_jar: CookieJar,
    Form(totp_setup_data): Form<TotpSetupFormData>,
) -> impl IntoResponse {
    match setup_totp(
        &state.database,
        &auth_data.user,
        &totp_setup_data.totp,
        state.config.auth.jwt_expiration_seconds,
        &state.config.auth.jwt_secret,
    )
    .await
    {
        Ok(result) => {
            let cookie_jar = cookie_jar.add(jwt_cookie(&result.jwt));

            (
                cookie_jar,
                Html(TotpRecoveryCodesPrompt::from(result).render()),
            )
                .into_response()
        }
        Err(error) => match error {
            TotpSetupError::TotpAlreadySetUp(..) => {
                redirect_response(is_hx_request, &bitsync_routes::GetFilesHomePage.to_string())
            }
            TotpSetupError::TotpCreation(..)
            | TotpSetupError::SystemTime(..)
            | TotpSetupError::TransactionBegin(..)
            | TotpSetupError::TransactionCommit(..)
            | TotpSetupError::Query(..)
            | TotpSetupError::PasswordHashCreation(..)
            | TotpSetupError::Jwt(..) => Html(
                TotpSetupForm {
                    // TODO: make optional
                    totp_secret_image_base64_img_src: String::new(),
                    totp_secret: String::new(),
                    error_message: Some(String::from(UNEXPECTED_ERROR_MESSAGE)),
                }
                .render(),
            )
            .into_response(),
            TotpSetupError::TotpInvalid(..) => {
                match retrieve_totp_setup_data(&auth_data.user).await {
                    Ok(totp_setup_data) => Html(
                        TotpSetupForm {
                            totp_secret_image_base64_img_src: totp_setup_data.secret_base64_qr_code,
                            totp_secret: totp_setup_data.secret_base32,
                            error_message: Some(String::from("The entered totp code is invalid.")),
                        }
                        .render(),
                    )
                    .into_response(),
                    Err(error) => {
                        Html(
                            RegisterPage::TotpSetup(TotpSetupForm {
                                // TODO: make optional
                                totp_secret_image_base64_img_src: String::new(),
                                totp_secret: String::new(),
                                error_message: Some(match error {
                                    RetrieveTotpSetupDataError::TotpAlreadySetUp(..) => {
                                        return redirect_response(
                                            is_hx_request,
                                            &bitsync_routes::GetFilesHomePage.to_string(),
                                        );
                                    }
                                    RetrieveTotpSetupDataError::TotpCreation(..)
                                    | RetrieveTotpSetupDataError::TotpSecretBase64QrCode(..) => {
                                        String::from(UNEXPECTED_ERROR_MESSAGE)
                                    }
                                }),
                            })
                            .render(),
                        )
                        .into_response()
                    }
                }
            }
        },
    }
}
