use std::sync::Arc;

use axum::{middleware::from_fn_with_state, response::IntoResponse, Router};
use axum_extra::extract::CookieJar;
use axum_extra::routing::RouterExt;

use crate::auth::require_login_middleware;

use crate::AppState;

use super::routes;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_get(logout_action_handler)
        .route_layer(from_fn_with_state(state.clone(), require_login_middleware))
}

async fn logout_action_handler(
    _: routes::GetLogoutAction,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let cookie_jar = cookie_jar.remove(crate::auth::AUTH_COOKIE_NAME);

    (
        cookie_jar,
        [("HX-Redirect", routes::GetLoginPage.to_string())],
    )
}
