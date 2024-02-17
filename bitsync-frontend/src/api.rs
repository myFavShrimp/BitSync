use self::http::{GraphQlError, RequestError};

static API_PATH: &str = "http://localhost:8080/api/graphql";

mod http;
pub mod public;

mod schema {
    #[cynic::schema("public")]
    pub mod public {}
    #[cynic::schema("private")]
    pub mod private {}
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ApiError {
    #[error(transparent)]
    Request(#[from] RequestError),
    #[error(transparent)]
    GraphQl(#[from] GraphQlError),
}
