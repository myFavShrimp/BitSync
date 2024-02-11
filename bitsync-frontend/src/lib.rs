use leptos::*;
use leptos_router::Router;

use crate::global_storage::use_login;

mod api;
mod global_storage;

#[component]
pub fn app() -> impl IntoView {
    let login = use_login();

    let (start, set_start) = create_signal(true);

    let res = create_resource(
        move || start.get(),
        |_| async move { api::query::login().await },
    );

    view! {
        <Router>
            "Hello, World!"
            <h1 on:click=move |_| {set_start.set(false)}>"go"</h1>
            {match login.get() {
                global_storage::LoginState::Invalid => String::from("Invalid"),
                global_storage::LoginState::Set(state) => state.sub.to_string(),
            }}
            {move || format!("{:?}", res.get())}
        </Router>
    }
}
