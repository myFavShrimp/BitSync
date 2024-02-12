static API_PATH: &str = "http://localhost:8080/api/graphql";

mod schema {
    #[cynic::schema("public")]
    pub mod public {}
    #[cynic::schema("private")]
    pub mod private {}
}

pub mod public;
