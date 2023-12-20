use std::{convert::Infallible, sync::Arc};

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{database::user::User, AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub sub: Uuid,
    pub exp: i64,
}

impl JwtClaims {
    fn decode(token: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let header = jsonwebtoken::decode_header(token)?;
        let mut validation = Validation::new(header.alg);
        validation.leeway = 0;

        jsonwebtoken::decode::<Self>(token, &decoding_key, &validation)
            .map(|token_data| token_data.claims)
    }

    pub fn encode(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let header = jsonwebtoken::Header::default();
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());

        let encoded_token = jsonwebtoken::encode::<Self>(&header, self, &encoding_key)?;
        Ok(format!("Bearer {encoded_token}"))
    }
}

#[derive(Debug, Clone)]
pub struct AuthData {
    pub claims: JwtClaims,
    pub user: User,
}

#[derive(Debug, Clone)]
pub enum AuthStatus {
    Missing,
    Invalid,
    User(AuthData),
}

impl AuthStatus {
    async fn from_token_str(token: &str, app_state: Arc<AppState>) -> Self {
        match JwtClaims::decode(token, &app_state.config.jwt_secret) {
            Ok(claims) => Self::for_claims(&app_state.postgres_pool, claims).await,
            Err(_) => Self::Invalid,
        }
    }

    async fn for_claims(connection: &PgPool, claims: JwtClaims) -> Self {
        match User::find_by_id(connection, &claims.sub).await {
            Err(_) => Self::Invalid,
            Ok(user) => Self::User(AuthData { claims, user }),
        }
    }
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
                    Self::from_token_str(bearer.token(), app_state).await
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
