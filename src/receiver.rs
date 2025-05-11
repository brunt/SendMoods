use leptos::prelude::{ElementChild, OnAttribute};
use leptos::{IntoView, component, logging, view};

#[component]
pub fn Receiver() -> impl IntoView {
    let submit_form = move |_| logging::log!("hi");
    let reset_form = move |_| logging::log!("hi");
    view! {
        <form on:submit=submit_form>
            <label for="text-blob">Paste your file text (blob):</label>
            <textarea id="text-blob" placeholder="Paste your text blob here..."></textarea>
            <input type="submit" value="Download" disabled />
            //             <input type="submit" disabled=move || !file_given.get() value="Generate Ticket" />
            <input on:click=reset_form type="reset" />
        </form>
    }
}
