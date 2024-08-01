use std::{convert::Infallible, sync::Arc};

use crate::{
    use_case::{self, auth::AuthData},
    AppState,
};
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;

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

        Ok(
            match TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await {
                Ok(TypedHeader(Authorization(bearer))) => {
                    if bearer.token().is_empty() {
                        return Ok(AuthStatus::Missing);
                    }

                    match use_case::auth::decode_auth_token(app_state, bearer.token()).await {
                        Ok(auth) => AuthStatus::User(auth),
                        Err(_) => AuthStatus::Invalid,
                    }
                }
                Err(e) => match e.reason() {
                    axum_extra::typed_header::TypedHeaderRejectionReason::Missing => {
                        AuthStatus::Missing
                    }
                    _ => AuthStatus::Invalid,
                },
            },
        )
    }
}
