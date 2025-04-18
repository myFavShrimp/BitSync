use std::sync::Arc;

use axum::{middleware::from_fn_with_state, response::IntoResponse, Router};
use axum_extra::extract::CookieJar;
use axum_extra::routing::RouterExt;
use axum_htmx::HxRequest;

use crate::auth::require_login_and_user_setup_middleware;

use crate::handler::redirect_response;
use crate::AppState;

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_get(logout_action_handler)
        .route_layer(from_fn_with_state(
            state.clone(),
            require_login_and_user_setup_middleware,
        ))
}

async fn logout_action_handler(
    _: bitsync_routes::GetLogoutAction,
    HxRequest(is_hx_request): HxRequest,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let cookie_jar = cookie_jar.remove(crate::auth::AUTH_COOKIE_NAME);

    (
        cookie_jar,
        redirect_response(is_hx_request, &bitsync_routes::GetLoginPage.to_string()),
    )
}
