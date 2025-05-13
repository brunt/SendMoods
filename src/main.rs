mod receiver;
mod sender;

use crate::receiver::Receiver;
use crate::sender::Sender;

use leptos::control_flow::Show;
use leptos::mount::mount_to_body;
use leptos::prelude::{Memo, window};
use leptos::{IntoView, component, view};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::UrlSearchParams;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let url = Memo::new(move |_| {
        window()
            .location()
            .href()
            .expect_throw("failed to get href")
    });

    // show the receiver component when there's a ticket in the querystring, otherwise show sender component
    let has_ticket = move || {
        let search = window()
            .location()
            .search()
            .expect("should have a search string");
        let search = search.trim_start_matches('?');
        let params =
            UrlSearchParams::new_with_str(search).expect_throw("failed to create UrlSearchParams");
        !params.has("ticket") //TODO: this negation makes the intended effect work but I don't yet see why
    };

    view! {
        <Show when=has_ticket fallback=move || view! { <Receiver /> }>
            <Sender url=url />
        </Show>
    }
}
