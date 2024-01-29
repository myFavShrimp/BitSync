use leptos::{component, view, IntoView};
use leptos_router::Router;

#[component]
pub fn app() -> impl IntoView {
    view! {
        <Router>
            "Hello, World!"
        </Router>
    }
}
