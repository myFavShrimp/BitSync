use std::{convert::Infallible, sync::Arc};

use crate::{database::user::User, jwt::JwtClaims, use_case, AppState};
use axum::{
    extract::{FromRef, FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use headers::Header;

pub static AUTH_COOKIE_NAME: &str = "auth";

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

        match CookieJar::from_request_parts(parts, state).await {
            Ok(cookie_jar) => match cookie_jar.get(AUTH_COOKIE_NAME) {
                Some(auth_cookie) => {
                    match use_case::auth::decode_auth_token(app_state, auth_cookie.value()).await {
                        Ok(auth) => Ok(auth),
                        Err(..) => Err(redirect_response(
                            &crate::handler::routes::GetLoginPage::route_path(),
                        )),
                    }
                }
                None => Err(redirect_response(
                    &crate::handler::routes::GetLoginPage::route_path(),
                )),
            },
            Err(..) => Err(redirect_response(
                &crate::handler::routes::GetLoginPage::route_path(),
            )),
        }
    }
}

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
                    match use_case::auth::decode_auth_token(app_state, auth_cookie.value()).await {
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

fn redirect_response(redirect_route: &str) -> Response {
    (
        StatusCode::SEE_OTHER,
        [
            ("HX-Redirect", redirect_route),
            (headers::Location::name().as_str(), redirect_route),
        ],
    )
        .into_response()
}

pub async fn require_logout_middleware(
    auth_status: AuthStatus,
    request: Request,
    next: Next,
) -> Response {
    match auth_status {
        AuthStatus::User(..) => {
            redirect_response(&crate::handler::routes::GetFilesHomePage::route_path())
        }
        AuthStatus::Missing | AuthStatus::Invalid => next.run(request).await,
    }
}

pub async fn require_login_middleware(
    _auth_data: AuthData,
    request: Request,
    next: Next,
) -> Response {
    next.run(request).await
}
