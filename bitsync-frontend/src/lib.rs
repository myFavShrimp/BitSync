use leptos::{html::Div, *};
use leptos_router::Router;

use global_storage::{provide_login_storage, use_login_state};
use leptos_use::{use_drop_zone_with_options, UseDropZoneOptions, UseDropZoneReturn};

use crate::{
    api::{
        private::{mutation::UploadUserFilesMutation, query::MeQuery},
        public::query::LoginQuery,
        GraphQlSendMutationOperationHelper, GraphQlSendQueryOperationHelper,
    },
    global_storage::use_login_token,
};

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

    let action_1 = LoginQuery::action();

    create_effect(move |_| {
        if let Some(Ok(value)) = action_1.value().get() {
            set_login.set(value.login);
        }
    });

    // me

    let action_2 = MeQuery::action();

    // file

    let action_3 = UploadUserFilesMutation::action();

    let drop_zone_el = create_node_ref::<Div>();

    let UseDropZoneReturn {
        is_over_drop_zone,
        files,
    } = use_drop_zone_with_options(drop_zone_el, UseDropZoneOptions::default());

    create_effect(move |_| {
        let files = files.get();
        if files.is_empty() {
            return;
        }
        let vars = api::private::mutation::UploadUserFilesMutationVariables {
            path: "/".to_string(),
            files: api::private::mutation::UploadFiles(files),
        };
        action_3.dispatch(vars);
    });

    let drop_zone_style = move || {
        format!(
            "border: 1px solid {}; width: 100px; height: 100px;",
            if is_over_drop_zone.get() {
                "red"
            } else {
                "blue"
            }
        )
    };

    view! {
        <Router>
            "Hello, World!"
            <h1 on:click=move |_| {action_1.dispatch(vars.clone())}>"login"</h1>
            <h1 on:click=move |_| {action_2.dispatch(())}>"me"</h1>
            <p>
                {move || format!("{:?}", login_state.get())}
            </p>
            <p>
                {move || format!("{:?}", action_1.value().get())}
            </p>
            <p>
                {move || format!("{:?}", action_2.value().get())}
            </p>

            <div style=drop_zone_style node_ref=drop_zone_el>
            </div>
        </Router>
    }
}
