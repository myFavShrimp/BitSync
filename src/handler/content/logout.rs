use std::sync::Arc;

use axum::routing::get;
use axum::{middleware::from_fn_with_state, response::IntoResponse, Router};
use axum_extra::extract::CookieJar;

use crate::auth::require_login_middleware;

use crate::AppState;

use super::routes;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            &routes::GetLogoutAction::handler_route(),
            get(logout_action_handler),
        )
        .route_layer(from_fn_with_state(state.clone(), require_login_middleware))
}

async fn logout_action_handler(cookie_jar: CookieJar) -> impl IntoResponse {
    let cookie_jar = cookie_jar.remove(crate::auth::AUTH_COOKIE_NAME);

    (
        cookie_jar,
        [("HX-Redirect", routes::GetLoginPage::route_path())],
    )
}
