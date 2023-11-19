#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod common_vec_op;
mod cus_component;
mod interfaces;
mod any_data;

use eframe::egui;
use crate::app::MainApp;


// beautiful colors
// c08eaf, fba414, 8cc269, 4f9da6, 9b5c5a, 5a5c9b, 9b5a5c, 5c9b5a, 5c9b9b, 9b5c9b
const COLOR_PALETTE: [egui::Color32; 10] = [
    egui::Color32::from_rgb(0xc0, 0x8e, 0xaf),
    egui::Color32::from_rgb(0xfb, 0xa4, 0x14),
    egui::Color32::from_rgb(0x8c, 0xc2, 0x69),
    egui::Color32::from_rgb(0x4f, 0x9d, 0xa6),
    egui::Color32::from_rgb(0x9b, 0x5c, 0x5a),
    egui::Color32::from_rgb(0x5a, 0x5c, 0x9b),
    egui::Color32::from_rgb(0x9b, 0x5a, 0x5c),
    egui::Color32::from_rgb(0x5c, 0x9b, 0x5a),
    egui::Color32::from_rgb(0x5c, 0x9b, 0x9b),
    egui::Color32::from_rgb(0x9b, 0x5c, 0x9b),
];

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1440.0, 960.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Vector Graphics Visualizer",
        options,
        Box::new(|_cc| Box::<MainApp>::default()),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "vegravis_canvas", // hardcode it
                web_options,
                Box::new(|_cc| Box::<MainApp>::default()),
            )
            .await
            .expect("failed to start eframe");
    });
}
