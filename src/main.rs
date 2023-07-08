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
    let mut sections: Vec<(String, String)> = vec![("".to_string(), "".to_string())];

    eframe::run_simple_native("code tweeter!!!", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("epic code tweeter");
            ui.vertical_centered(|ui| {
                ui.heading("epic code tweeter");
            });

            ScrollArea::vertical()
                .min_scrolled_height(_frame.info().window_info.size.y)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.heading("text");
                            ui.heading("code");
                            ui.heading("preview");
                        });

                        for b in &mut sections {
                            ui.horizontal(|ui| {
                                ui.text_edit_multiline(&mut b.0);
                                ui.text_edit_multiline(&mut b.1);

                                let x = &b.0;
                                ui.label(format!("your text: {x}"));
                            });
                        }

                        // Submit button
                        let add_button: egui::Response = ui.button("Add Section");
                        if add_button.clicked() {
                            sections.push(("caption...".to_string(), "".to_string()));
                        }
                    });
                });
        });
    })
}
