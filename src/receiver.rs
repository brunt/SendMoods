use leptos::prelude::{ElementChild, OnAttribute};
use leptos::{IntoView, component, logging, view};
use leptos::prelude::*;

#[component]
pub fn Receiver() -> impl IntoView {
    let submit_form = move |_| logging::log!("hi");
    let reset_form = move |_| logging::log!("bye");
    view! {
        <p class="mb-6">You are downloading <b class="text-blue-500">Thinking.epub</b>.</p>
        <form class="flex gap-4 mb-6" on:submit=submit_form>
            <input type="submit" value="Download" class="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-6 rounded disabled:bg-gray-600 disabled:cursor-not-allowed transition-colors duration-200" disabled />
            // <input type="submit" disabled=move || !file_given.get() value="Generate Ticket" />
            <input class="bg-gray-700 hover:bg-gray-600 text-gray-200 font-semibold py-2 px-6 rounded transition-colors duration-200" on:click=reset_form type="reset" />
        </form>
    }
}
