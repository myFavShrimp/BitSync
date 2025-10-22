use std::sync::Arc;

use axum::{Router, middleware::from_fn_with_state, response::IntoResponse};
use axum_extra::extract::CookieJar;
use axum_extra::routing::RouterExt;

use crate::auth::require_login_and_totp_setup_middleware;

use crate::AppState;
use crate::handler::{RedirectHttp, http_redirect_response};

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_get(logout_action_handler)
        .route_layer(from_fn_with_state(
            state.clone(),
            require_login_and_totp_setup_middleware::<RedirectHttp>,
        ))
}

async fn logout_action_handler(
    _: bitsync_routes::GetLogoutAction,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let cookie_jar = cookie_jar.remove(crate::auth::AUTH_COOKIE_NAME);

    (
        cookie_jar,
        http_redirect_response(&bitsync_routes::GetLoginPage.to_string()),
    )
}
