use fast_qr::QRBuilder;
use fast_qr::convert::svg::SvgBuilder;
use fast_qr::convert::{Builder, Shape};
use leptos::html::Input;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos::{IntoView, component, logging, view};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{DragEvent, Event, MouseEvent, SubmitEvent, js_sys};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (qr, set_qr) = signal(String::new());
    let (file_given, set_file_given) = signal(false);
    let url = Memo::new(move |_| {
        window()
            .location()
            .href()
            .expect_throw("failed to get href")
    });

    let hidden_input: NodeRef<Input> = NodeRef::new();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        // generate a qr code with the value being the full url of this page
        let full_url = format!("{}?ticket={}", url.get(), name.get());
        let qrcode = QRBuilder::new(full_url)
            .build()
            .expect_throw("failed to build QR");
        let svgqr = SvgBuilder::default().shape(Shape::Square).to_str(&qrcode);
        set_qr.set(svgqr);

        window()
            .history()
            .expect_throw("failed to get history")
            .push_state_with_url(
                &js_sys::Object::new(),
                "",
                Some(&format!("?ticket={}", name.get())),
            )
            .expect_throw("failed to set querystring");
    };

    let on_drop = move |ev: DragEvent| {
        ev.prevent_default();

        let file = ev
            .data_transfer()
            .expect_throw("failed to get data transfer")
            .files()
            .expect_throw("getting files from data transfer")
            .get(0)
            .expect_throw("failed to get file after getting files after getting data transfer");
        set_name.set(file.name());
        logging::log!("{:?}", file);
    };

    let on_dragover = move |ev: DragEvent| {
        ev.prevent_default();
        let var = ev.target().unwrap();
        logging::log!("{}", var.to_string());
    };
    let div_on_click = move |ev: MouseEvent| {
        ev.prevent_default();

        if let Some(input) = hidden_input.get() {
            input.click();
        }
    };

    let on_input_change = move |_event: Event| {
        let fileList = hidden_input.get().expect_throw("input get").files();
        let file = fileList
            .expect_throw("files")
            .item(0)
            .expect_throw("getting file 0");
        set_name.set(file.name());
        set_file_given.set(true);
    };

    let reset_form = move |_ev: MouseEvent| {
        set_name.set(String::new());
        set_qr.set(String::new());
        set_file_given.set(false);
        window()
            .history()
            .expect_throw("failed to get history")
            .push_state_with_url(&js_sys::Object::new(), "", Some(&url.get()))
            .expect_throw("failed to set querystring");
    };
    view! {
        <div>
            <div id="drop-area" on:drop=on_drop on:dragover=on_dragover on:click=div_on_click>
                {move || {
                    let text = name.read().to_string();
                    if text.is_empty() { "Drag a file here".to_string() } else { text }
                }}
            </div>
            <input type="file" node_ref=hidden_input hidden on:change=on_input_change />
        </div>
        <form on:submit=on_submit>
            <input type="submit" disabled=move || !file_given.get() value="Generate Ticket" />
            <input on:click=reset_form type="reset" />
        </form>
        <div id="qr-code" inner_html=move || qr.get()></div>
    }
}
