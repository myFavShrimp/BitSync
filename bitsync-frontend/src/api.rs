use std::rc::Rc;

static API_PATH: &str = "http://localhost:8080/api/graphql";

mod schema {
    #[cynic::schema("public")]
    pub mod public {}
    #[cynic::schema("private")]
    pub mod private {}
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ApiError {
    #[error(transparent)]
    Gloo(#[from] Rc<gloo_net::Error>),
    #[error(transparent)]
    Json(#[from] Rc<serde_json::Error>),
}

pub mod public;
