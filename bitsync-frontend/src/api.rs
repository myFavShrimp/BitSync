use self::http::{GraphQlError, RequestError};

static API_PATH: &str = "http://localhost:8080/api/graphql";

mod http;
pub mod private;
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

pub type GraphQlResult<T, E = ApiError> = core::result::Result<T, E>;

pub trait GraphQlOperationHelper<V>
where
    Self: Sized,
{
    async fn send(variables: V) -> GraphQlResult<Self>;
    fn action() -> leptos::Action<V, GraphQlResult<Self>>;
}

impl<T, V> GraphQlOperationHelper<V> for T
where
    T: cynic::QueryBuilder<V> + for<'de> serde::Deserialize<'de>,
    V: serde::Serialize + Clone,
{
    async fn send(variables: V) -> GraphQlResult<Self> {
        let operation = Self::build(variables);

        http::post_graphql_operation(operation).await
    }

    fn action() -> leptos::Action<V, GraphQlResult<Self>> {
        leptos::create_action(|vars: &V| Self::send(vars.clone()))
    }
}
