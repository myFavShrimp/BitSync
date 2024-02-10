use jwt::JwtClaims;
use leptos::{create_effect, signal_prelude::*};
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};

static JWT_STORAGE_KEY: &str = "JWT";

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum LoginState {
    Invalid,
    Set(JwtClaims),
}

pub fn use_login() -> Memo<LoginState> {
    let (login, _set_login, _) = use_local_storage::<String, FromToStringCodec>(JWT_STORAGE_KEY);

    let login_state = create_memo(move |_| match JwtClaims::decode(&login.get()) {
        Ok(claims) => LoginState::Set(claims),
        Err(_) => LoginState::Invalid,
    });

    create_effect(move |_| {
        tracing::debug!("{:#?}", login.get());
    });

    login_state
}
