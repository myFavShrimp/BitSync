use jwt::JwtClaims;
use leptos::{provide_context, signal_prelude::*, use_context};
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};

static JWT_STORAGE_KEY: &str = "JWT";

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum LoginState {
    Invalid,
    Set(JwtClaims),
}

type LoginStateSignals = (Memo<LoginState>, WriteSignal<String>);

#[derive(Clone)]
pub struct GlobalLoginStorage(LoginStateSignals);

impl GlobalLoginStorage {
    pub fn provide() {
        provide_context(Self(Self::initialize_signals()))
    }

    fn initialize_signals() -> LoginStateSignals {
        let (login, set_login, _) = use_local_storage::<String, FromToStringCodec>(JWT_STORAGE_KEY);

        let login_state = create_memo(move |_| match JwtClaims::decode(&login.get()) {
            Ok(claims) => LoginState::Set(claims),
            Err(_) => LoginState::Invalid,
        });

        (login_state, set_login)
    }
}

pub fn use_login_state() -> LoginStateSignals {
    let login_storage =
        use_context::<GlobalLoginStorage>().expect("GlobalLoginStorage is initialized");

    login_storage.0
}
