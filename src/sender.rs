use fast_qr::QRBuilder;
use fast_qr::convert::svg::SvgBuilder;
use fast_qr::convert::{Builder, Shape};
use leptos::html::Input;
use leptos::prelude::*;
use leptos::{IntoView, component, logging, view};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{DragEvent, Event, File, MouseEvent, SubmitEvent, js_sys};

#[component]
pub fn Sender() -> impl IntoView {
    let (file_signal, set_file_signal) = signal_local::<Option<File>>(None);
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

        // use iroh to generate a blob for the file(s) given
        let blob = generate_blob(file_signal.get().unwrap());
        // let blob = generate_blob(&name.get());

        // generate a qr code with the value being the full url of this page
        let full_url = format!("{}?ticket={}", url.get(), blob);
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
                Some(&format!("?ticket={}", blob)),
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
        set_file_signal.set(Some(file));
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
        set_file_signal.set(Some(file));
        set_file_given.set(true);
    };

    let reset_form = move |_ev: MouseEvent| {
        set_file_signal.set(None);
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
                    file_signal.get().map_or("Drag a file here".to_string(), |file| file.name())
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

// generate an iroh blob from a file
fn generate_blob(input: File) -> String {
    logging::log!("generate blob {:?}", input);
    "blob123abcdef000000000000000000000000000000000000000000000000".into()
}
