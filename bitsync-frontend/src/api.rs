use self::http::WithOptionalFileMapper;

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
    #[error("Sending the GraphQL query failed")]
    Request(#[from] http::RequestError),
    #[error("Handling the GraphQL response failed")]
    Response(#[from] http::ResponseError),
}

pub type GraphQlResult<T> = core::result::Result<T, ApiError>;

pub trait GraphQlSendQueryOperationHelper<V>
where
    Self: Sized,
{
    async fn send(variables: V) -> GraphQlResult<Self>;
    fn action() -> leptos::Action<V, GraphQlResult<Self>>;
}

impl<T, V> GraphQlSendQueryOperationHelper<V> for T
where
    T: cynic::QueryBuilder<V> + for<'de> serde::Deserialize<'de>,
    V: serde::Serialize + Clone + WithOptionalFileMapper,
{
    async fn send(variables: V) -> GraphQlResult<Self> {
        let operation = T::build(variables);

        http::post_multipart_graphql_operation(&operation).await
    }

    fn action() -> leptos::Action<V, GraphQlResult<Self>> {
        leptos::create_action(|vars: &V| Self::send(vars.clone()))
    }
}

pub trait GraphQlSendMutationOperationHelper<V>
where
    Self: Sized,
{
    async fn send(variables: V) -> GraphQlResult<Self>;
    fn action() -> leptos::Action<V, GraphQlResult<Self>>;
}

impl<T, V> GraphQlSendMutationOperationHelper<V> for T
where
    T: cynic::MutationBuilder<V> + for<'de> serde::Deserialize<'de>,
    V: serde::Serialize + Clone + WithOptionalFileMapper,
{
    async fn send(variables: V) -> GraphQlResult<Self> {
        let operation = T::build(variables);

        http::post_multipart_graphql_operation(&operation).await
    }

    fn action() -> leptos::Action<V, GraphQlResult<Self>> {
        leptos::create_action(|vars: &V| Self::send(vars.clone()))
    }
}
