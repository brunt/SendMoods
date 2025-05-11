mod receiver;
mod sender;

use crate::receiver::Receiver;
use crate::sender::Sender;

use leptos::mount::mount_to_body;
use leptos::{IntoView, component,view};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Sender />
        <Receiver />
    }
}
