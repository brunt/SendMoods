use leptos::prelude::*;
use leptos::prelude::{ElementChild, OnAttribute};
use leptos::{IntoView, component, logging, view};
use web_sys::{MouseEvent, SubmitEvent};

#[component]
pub fn Receiver(#[prop(into)] ticket: Option<String>) -> impl IntoView {
    let (text_area, set_text_area) = signal(ticket.unwrap_or_default());
    let submit_form = move |e: SubmitEvent| {
        e.prevent_default();
        logging::log!("submitted form");
        // TODO: when iroh has wasm support, we download a blob
    };

    let reset_form = move |_ev: MouseEvent| {
        set_text_area.set(String::new());
    };

    //TODO: <A/> component that links to sender
    view! {
        <textarea class="w-96 m-6 truncate hover:text-clip text-slate-800 break-words overflow-auto rounded">
            {move || text_area.get()}
        </textarea>
        <p class="mb-6">You are downloading <b class="text-blue-500">Thinking.epub</b>.</p>
        <form class="flex gap-4 mb-6" on:submit=submit_form>
            <input
                type="submit"
                value="Download"
                class="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-6 rounded disabled:bg-gray-600 disabled:cursor-not-allowed transition-colors duration-200"
                disabled
            />
            <input
                class="bg-gray-700 hover:bg-gray-600 text-gray-200 font-semibold py-2 px-6 rounded transition-colors duration-200"
                on:click=reset_form
                type="reset"
            />
        </form>
    }
}
