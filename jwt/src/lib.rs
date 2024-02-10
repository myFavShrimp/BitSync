use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use jsonwebtoken;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct JwtClaims {
    pub sub: Uuid,
    pub exp: i64,
}

impl JwtClaims {
    pub fn decode_and_validate(
        token: &str,
        secret: &str,
    ) -> Result<Self, jsonwebtoken::errors::Error> {
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let header = jsonwebtoken::decode_header(token)?;
        let mut validation = Validation::new(header.alg);
        validation.leeway = 0;

        jsonwebtoken::decode::<Self>(token, &decoding_key, &validation)
            .map(|token_data| token_data.claims)
    }

    pub fn decode(token: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let invalid_decoding_key = DecodingKey::from_secret(&[]);
        let header = jsonwebtoken::decode_header(token)?;
        let mut validation = Validation::new(header.alg);
        validation.insecure_disable_signature_validation();

        jsonwebtoken::decode::<Self>(token, &invalid_decoding_key, &validation)
            .map(|token_data| token_data.claims)
    }

    pub fn encode(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let header = jsonwebtoken::Header::default();
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());

        let encoded_token = jsonwebtoken::encode::<Self>(&header, self, &encoding_key)?;
        Ok(format!("Bearer {encoded_token}"))
    }
}
