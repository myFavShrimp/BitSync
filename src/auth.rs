use std::{convert::Infallible, sync::Arc};

use crate::{handler::redirect_response, AppState};
use axum::{
    extract::{FromRef, FromRequestParts, Request},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use axum_htmx::HxRequest;
use bitsync_core::jwt::JwtClaims;
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

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for AuthData
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = Arc::<AppState>::from_ref(state);
        let HxRequest(is_hx_request) = match HxRequest::from_request_parts(parts, state).await {
            Ok(is_hx_request) => is_hx_request,
            Err(infallible) => match infallible {},
        };

        match CookieJar::from_request_parts(parts, state).await {
            Ok(cookie_jar) => match cookie_jar.get(AUTH_COOKIE_NAME) {
                Some(auth_cookie) => {
                    match decode_auth_token(app_state, auth_cookie.value()).await {
                        Ok(auth) => Ok(auth),
                        Err(..) => Err(redirect_response(
                            is_hx_request,
                            &crate::handler::routes::GetLoginPage.to_string(),
                        )),
                    }
                }
                None => Err(redirect_response(
                    is_hx_request,
                    &crate::handler::routes::GetLoginPage.to_string(),
                )),
            },
            Err(..) => Err(redirect_response(
                is_hx_request,
                &crate::handler::routes::GetLoginPage.to_string(),
            )),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AuthStatus {
    Missing,
    Invalid,
    User(AuthData),
}

#[async_trait::async_trait]
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
        AuthStatus::User(..) => redirect_response(
            is_hx_request,
            &crate::handler::routes::GetFilesHomePage.to_string(),
        ),
        AuthStatus::Missing | AuthStatus::Invalid => next.run(request).await,
    }
}

pub async fn require_login_middleware(
    auth_status: AuthStatus,
    HxRequest(is_hx_request): HxRequest,
    request: Request,
    next: Next,
) -> Response {
    match auth_status {
        AuthStatus::Missing | AuthStatus::Invalid => redirect_response(
            is_hx_request,
            &crate::handler::routes::GetLoginPage.to_string(),
        ),
        AuthStatus::User(..) => next.run(request).await,
    }
}
