use leptos::{component, signal_prelude::*, view, IntoView};
use leptos_router::Router;

use crate::global_storage::use_login;

mod global_storage;

#[component]
pub fn app() -> impl IntoView {
    let login = use_login();

    view! {
        <Router>
            "Hello, World!"
            {match login.get() {
                global_storage::LoginState::Invalid => String::from("Invalid"),
                global_storage::LoginState::Set(state) => state.sub.to_string(),
            }}
        </Router>
    }
}
