#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, ScrollArea};

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let window_size = 640.0;

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(window_size, window_size)),
        ..Default::default()
    };

    // Our application state:
    let mut boxes: Vec<String> = vec!["".to_string()];

    eframe::run_simple_native("code tweeter!!!", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("epic code tweeter");
            ui.vertical_centered(|ui| {
                ui.heading("epic code tweeter");
            });

            ui.horizontal(|ui| {
                ScrollArea::vertical()
                    .min_scrolled_height(window_size)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            for b in &mut boxes {
                                ui.text_edit_multiline(b);
                            }
                            
                            // Submit button
                            let add_button: egui::Response = ui.button("Add Text");
                            if add_button.clicked() {
                                boxes.push("default value".to_string());
                            }
                        });
                    });
                let code = &boxes.concat();
                ui.label(format!("your text: {code}"));
            });
        });
    })
}
