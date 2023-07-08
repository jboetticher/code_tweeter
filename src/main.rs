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
    let mut code: String = "Arthur".to_owned();

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
                            ui.text_edit_multiline(&mut code);
                            // Submit button
                            let submit_button: egui::Response = ui.button("Submit");
                            if submit_button.clicked() {
                                println!("stop clicking me uwu");
                            }
                        });
                    });
                ui.label(format!("your text: {code}"));
            });
        });
    })
}
