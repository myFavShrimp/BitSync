use bitsync_jwt::JwtClaims;
use leptos::{provide_context, signal_prelude::*, use_context};
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};

static JWT_STORAGE_KEY: &str = "JWT";

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum LoginState {
    Invalid,
    NotSet,
    Set(JwtClaims),
}

#[derive(Clone)]
struct GlobalLoginStorage {
    login_state: Memo<LoginState>,
    login: Signal<String>,
    set_login: WriteSignal<String>,
}

impl GlobalLoginStorage {
    fn new() -> Self {
        let (login, set_login, _) = use_local_storage::<String, FromToStringCodec>(JWT_STORAGE_KEY);

        let login_state = create_memo(move |_| {
            let login_token = login.get();

            if login_token.is_empty() {
                LoginState::NotSet
            } else {
                match JwtClaims::decode(&login.get()) {
                    Ok(claims) => LoginState::Set(claims),
                    Err(_) => LoginState::Invalid,
                }
            }
        });

        Self {
            login_state,
            login,
            set_login,
        }
    }
}

pub fn provide_login_storage() {
    provide_context(GlobalLoginStorage::new())
}

pub fn use_login_state() -> Memo<LoginState> {
    let login_storage =
        use_context::<GlobalLoginStorage>().expect("GlobalLoginStorage is initialized");

    login_storage.login_state
}

pub fn use_login_token() -> (Signal<String>, WriteSignal<String>) {
    let login_storage =
        use_context::<GlobalLoginStorage>().expect("GlobalLoginStorage is initialized");

    (login_storage.login, login_storage.set_login)
}
