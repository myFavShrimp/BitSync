use bitsync_jwt::JwtClaims;
use leptos::{create_effect, provide_context, signal_prelude::*, use_context};
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};

use crate::api::{
    private::query::{MeQuery, User},
    GraphQlSendQueryOperationHelper,
};

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
    current_user: Memo<Option<User>>,
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

        let me_query_action = MeQuery::action();

        create_effect(move |_| {
            let _ = login.get();
            me_query_action.dispatch(());
        });

        let current_user = create_memo(move |_| match me_query_action.value().get() {
            Some(Ok(query_result)) => Some(query_result.me),
            None | Some(Err(_)) => None,
        });

        Self {
            login_state,
            login,
            set_login,
            current_user,
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

pub fn use_current_user() -> Memo<Option<User>> {
    let login_storage =
        use_context::<GlobalLoginStorage>().expect("GlobalLoginStorage is initialized");

    login_storage.current_user
}
