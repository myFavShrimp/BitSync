use leptos::*;
use leptos_router::Router;

use global_storage::{provide_login_storage, use_login_state};

use crate::global_storage::use_login_token;

mod api;
mod global_storage;

#[component]
pub fn app() -> impl IntoView {
    provide_login_storage();
    let login_state = use_login_state();

    // login

    let (_login, set_login) = use_login_token();

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

    // me

    let res_2 = create_action(|()| api::private::query::me());

    view! {
        <Router>
            "Hello, World!"
            <h1 on:click=move |_| {res.dispatch(vars.clone())}>"login"</h1>
            <h1 on:click=move |_| {res_2.dispatch(())}>"me"</h1>
            <p>
                {move || match login_state.get() {
                    global_storage::LoginState::Invalid => String::from("Invalid"),
                    global_storage::LoginState::NotSet => String::from("Not Set"),
                    global_storage::LoginState::Set(state) => state.sub.to_string(),
                }}
            </p>
            <p>
                {move || format!("{:?}", res.value().get())}
            </p>
            <p>
                {move || format!("{:?}", res_2.value().get())}
            </p>
        </Router>
    }
}
