use std::sync::Arc;

use axum::{
    Extension, Router, extract::State, middleware::from_fn_with_state, response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use axum_extra::routing::RouterExt;
use bitsync_database::repository;

use crate::auth::{AuthData, require_login_and_totp_setup_middleware};

use crate::AppState;
use crate::handler::{RedirectHttp, http_redirect_response};

pub(crate) async fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .typed_get(logout_action_handler)
        .route_layer(from_fn_with_state(
            state.clone(),
            require_login_and_totp_setup_middleware::<RedirectHttp>,
        ))
        .with_state(state)
}

async fn logout_action_handler(
    _: bitsync_routes::GetLogoutAction,
    State(state): State<Arc<AppState>>,
    Extension(auth_data): Extension<AuthData>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    if let Ok(mut connection) = state.database.acquire_connection().await {
        let _ = repository::session::delete_by_id(&mut *connection, &auth_data.session.id).await;
    }

    let cookie_jar = cookie_jar.remove(crate::auth::AUTH_COOKIE_NAME);

    (
        cookie_jar,
        http_redirect_response(&bitsync_routes::GetLoginPage.to_string()),
    )
}
