mod any_data;
mod app;
mod common_vec_op;
mod cus_component;
mod interfaces;
mod sample_codes_list;

pub use app::MainApp;
pub use eframe::egui;

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
