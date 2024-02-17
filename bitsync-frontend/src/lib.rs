use global_storage::use_login_state;
use leptos::*;
use leptos_router::Router;

use crate::global_storage::GlobalLoginStorage;

mod api;
mod global_storage;

#[component]
pub fn app() -> impl IntoView {
    GlobalLoginStorage::provide();
    let (login, set_login) = use_login_state();

    let vars = api::public::query::LoginQueryVariables {
        username: String::from("test"),
        password: String::from("test"),
    };

    let res = create_action(|input: &api::public::query::LoginQueryVariables| {
        api::public::query::login(input.clone())
    });

    create_effect(move |_| {
        if let Some(Ok(value)) = res.value().get() {
            set_login.set(value.login);
        }
    });

    view! {
        <Router>
            "Hello, World!"
            <h1 on:click=move |_| {res.dispatch(vars.clone())}>"go"</h1>
            <p>
                {move || match login.get() {
                    global_storage::LoginState::Invalid => String::from("Invalid"),
                    global_storage::LoginState::Set(state) => state.sub.to_string(),
                }}
            </p>
            <p>
            {move || format!("{:?}", res.value().get())}
            </p>
        </Router>
    }
}
