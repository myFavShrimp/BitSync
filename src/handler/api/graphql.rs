use std::sync::Arc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::{auth::AuthStatus, AppState};

use super::routes;

mod dataloader;
mod schema;

pub use schema::private::{
    create_root as create_private_root, Context as PrivateContext, Root as PrivateRoot,
};
pub use schema::public::{
    create_root as create_public_root, Context as PublicContext, Root as PublicRoot,
};

pub async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            &routes::ApiGraphQl::handler_route(),
            get(api_graphql_get_handler).post(api_graphql_post_handler),
        )
        .with_state(state)
}

pub async fn api_graphql_get_handler() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        &routes::ApiGraphQl::route_path(),
    )))
}

pub async fn api_graphql_post_handler(
    State(state): State<Arc<AppState>>,
    auth_status: AuthStatus,
    req: GraphQLRequest,
) -> GraphQLResponse {
    match auth_status {
        AuthStatus::Missing | AuthStatus::Invalid => {
            let context = PublicContext {
                app_state: state.clone(),
            };

            state
                .public_graphql_api_schema
                .execute(req.into_inner().data(context))
                .await
                .into()
        }
        AuthStatus::User(_user) => {
            let context = PrivateContext {
                app_state: state.clone(),
            };

            state
                .private_graphql_api_schema
                .execute(req.into_inner().data(context))
                .await
                .into()
        }
    }
}
