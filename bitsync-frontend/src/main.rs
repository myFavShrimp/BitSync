fn main() {
    _ = console_log::init_with_level(tracing::log::Level::Debug);
    console_error_panic_hook::set_once();

    tracing::debug!("Hello, World!");

    leptos::mount_to_body(|| leptos::view! { <App/> })
}

#[leptos::component]
fn app() -> impl leptos::IntoView {
    leptos::view! {
        <h1>"Hello, World!"</h1>
    }
}
