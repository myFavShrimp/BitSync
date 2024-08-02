use std::{convert::Infallible, sync::Arc};

use crate::{
    use_case::{self, auth::AuthData},
    AppState,
};
use axum::{
    extract::{FromRef, FromRequest, FromRequestParts, Request, State},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;

pub static AUTH_COOKIE_NAME: &str = "auth";

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

pub struct RequireLogin;

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for RequireLogin
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match AuthStatus::from_request_parts(parts, state).await {
            Ok(AuthStatus::User(..)) => Ok(Self),
            Ok(..) | Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    }
}
