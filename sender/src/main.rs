use eframe::egui;
use std::path::PathBuf;
// use clipboard_rs::{ClipboardContext, ClipboardProvider};

#[derive(Default)]
struct MyApp {
    file_path: Option<PathBuf>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Instructions");

            ui.label("Drag a single file into the window to generate a ticket. Copy and send the ticket to someone so they can receive the file.");
            ui.add_space(4.0);
            // Create a text input area to show the path to the dragged file
            if let Some(file_path) = &self.file_path {
                ui.horizontal(|ui| {
                    ui.label(format!("File path:{file_path:?}"));
                });
                ui.end_row();
                ui.add_space(4.0);
                // if ui.button("Copy").clicked() {
                //     let mut cliboard_ctx : ClipboardContext = ClipboardProvider::new().unwrap();
                //     cliboard_ctx.set_contents(path.clone()).unwrap();
                // }
            } else {
                ui.label("No file selected");
            }

            // Handle drag-and-drop events
            ctx.input(|input_state| {
                if !input_state.raw.dropped_files.is_empty() {
                    for dropped_file in &input_state.raw.dropped_files {
                        if let Some(path) = &dropped_file.path {
                            dbg!(path);
                            self.file_path = Some(path.display().to_string().parse().unwrap());
                        }
                    }
                }
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Send Moods",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(600.0, 400.0)),
            renderer: eframe::Renderer::Wgpu, // Glow renderer doesn't work for me rn
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::<MyApp>::new(MyApp::default()))),
    )
}
