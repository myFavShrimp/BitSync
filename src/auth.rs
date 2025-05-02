use std::{convert::Infallible, sync::Arc};

use crate::{handler::redirect_response, AppState};
use axum::{
    extract::{FromRef, FromRequestParts, Request},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use axum_htmx::HxRequest;
use bitsync_core::jwt::{JwtClaims, LoginState};
use bitsync_database::{database::ConnectionAcquisitionError, entity::User, repository};

#[derive(Debug, thiserror::Error)]
#[error("The provided auth token is invalid")]
pub enum AuthTokenDecodeError {
    Database(#[from] repository::QueryError),
    ConnectionAcquisition(#[from] ConnectionAcquisitionError),
    Decode(#[from] bitsync_core::jwt::Error),
}

async fn decode_auth_token(
    app_state: Arc<AppState>,
    token: &str,
) -> Result<AuthData, AuthTokenDecodeError> {
    let mut connection = app_state.database.acquire_connection().await?;

    let claims = JwtClaims::decode_and_validate(token, &app_state.config.auth.jwt_secret)?;
    let user = repository::user::find_by_id(&mut *connection, &claims.sub).await?;

    Ok(AuthData { claims, user })
}

pub static AUTH_COOKIE_NAME: &str = "auth";

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AuthData {
    pub claims: JwtClaims,
    pub user: User,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AuthStatus {
    Missing,
    Invalid,
    User(AuthData),
}

impl<S> FromRequestParts<S> for AuthStatus
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = Arc::<AppState>::from_ref(state);

        Ok(match CookieJar::from_request_parts(parts, state).await {
            Ok(cookie_jar) => match cookie_jar.get(AUTH_COOKIE_NAME) {
                Some(auth_cookie) => {
                    match decode_auth_token(app_state, auth_cookie.value()).await {
                        Ok(auth) => AuthStatus::User(auth),
                        Err(..) => AuthStatus::Invalid,
                    }
                }
                None => AuthStatus::Missing,
            },
            Err(..) => AuthStatus::Missing,
        })
    }
}

pub async fn require_logout_middleware(
    auth_status: AuthStatus,
    HxRequest(is_hx_request): HxRequest,
    request: Request,
    next: Next,
) -> Response {
    match auth_status {
        AuthStatus::Missing | AuthStatus::Invalid => next.run(request).await,
        AuthStatus::User(..) => {
            redirect_response(is_hx_request, &bitsync_routes::GetFilesHomePage.to_string())
        }
    }
}

pub async fn require_login_and_totp_setup_middleware(
    auth_status: AuthStatus,
    HxRequest(is_hx_request): HxRequest,
    mut request: Request,
    next: Next,
) -> Response {
    match auth_status {
        AuthStatus::Missing | AuthStatus::Invalid => {
            redirect_response(is_hx_request, &bitsync_routes::GetLoginPage.to_string())
        }
        AuthStatus::User(auth_data) => {
            if !auth_data.user.is_totp_set_up {
                return redirect_response(
                    is_hx_request,
                    &bitsync_routes::GetRegisterTotpSetupPage.to_string(),
                );
            }
            if auth_data.claims.login_state == LoginState::Basic {
                return redirect_response(
                    is_hx_request,
                    &bitsync_routes::GetLoginTotpAuthPage.to_string(),
                );
            }

            let extensions = request.extensions_mut();
            extensions.insert(auth_data);

            next.run(request).await
        }
    }
}

pub async fn require_basic_login_and_totp_setup_middleware(
    auth_status: AuthStatus,
    HxRequest(is_hx_request): HxRequest,
    mut request: Request,
    next: Next,
) -> Response {
    match auth_status {
        AuthStatus::Missing | AuthStatus::Invalid => {
            redirect_response(is_hx_request, &bitsync_routes::GetLoginPage.to_string())
        }
        AuthStatus::User(auth_data) => {
            if !auth_data.user.is_totp_set_up {
                return redirect_response(
                    is_hx_request,
                    &bitsync_routes::GetRegisterTotpSetupPage.to_string(),
                );
            }
            if auth_data.claims.login_state == LoginState::Full {
                return redirect_response(
                    is_hx_request,
                    &bitsync_routes::GetFilesHomePage.to_string(),
                );
            }

            let extensions = request.extensions_mut();
            extensions.insert(auth_data);

            next.run(request).await
        }
    }
}

pub async fn require_login_and_no_totp_setup_middleware(
    auth_status: AuthStatus,
    HxRequest(is_hx_request): HxRequest,
    mut request: Request,
    next: Next,
) -> Response {
    match auth_status {
        AuthStatus::Missing | AuthStatus::Invalid => {
            redirect_response(is_hx_request, &bitsync_routes::GetLoginPage.to_string())
        }
        AuthStatus::User(auth_data) => {
            if auth_data.user.is_totp_set_up && auth_data.claims.login_state == LoginState::Basic {
                return redirect_response(
                    is_hx_request,
                    &bitsync_routes::GetLoginTotpAuthPage.to_string(),
                );
            }

            let extensions = request.extensions_mut();
            extensions.insert(auth_data);

            next.run(request).await
        }
    }
}

pub fn jwt_cookie<'a>(jwt: &str) -> Cookie<'a> {
    let mut auth_cookie =
        axum_extra::extract::cookie::Cookie::new(crate::auth::AUTH_COOKIE_NAME, jwt.to_owned());
    auth_cookie.set_same_site(SameSite::Strict);
    auth_cookie.set_path("/");

    #[cfg(not(debug_assertions))]
    auth_cookie.set_secure(true);

    auth_cookie
}
