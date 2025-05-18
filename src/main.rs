mod receiver;
mod sender;

use crate::receiver::Receiver;
use crate::sender::Sender;

use leptos::control_flow::Show;
use leptos::mount::mount_to_body;
use leptos::prelude::Read;
use leptos::{IntoView, component, view};
use leptos_router::components::{Router};
use leptos_router::hooks::use_query_map;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <AppButInsideRouter />
        </Router>
    }
}

#[component]
pub fn AppButInsideRouter() -> impl IntoView {
    let query = use_query_map();
    let ticket = move || query.read().get("ticket");

    view! {
        <Show when=move || ticket().is_some() fallback=move || view! { <Sender /> }>
            <Receiver ticket=ticket() />
        </Show>
    }
}
