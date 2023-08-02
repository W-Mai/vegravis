#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod syntax;

use eframe::{egui};
use eframe::egui::{Sense, Vec2, Widget};
use egui_code_editor::{CodeEditor, ColorTheme};
use egui_extras::{Size, StripBuilder};
use crate::syntax::vec_op_syntax;

const DEFAULT_CODE: &str = include_str!("default_code");

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(960.0, 640.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Vector Graphics Visualizer",
        options,
        Box::new(|_cc| Box::<MainApp>::default()),
    )
}

struct MainApp {
    code: String,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            code: DEFAULT_CODE.to_owned(),
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::exact(30.0))
                .size(Size::remainder())
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Vector Graphics Visualizer");
                        });
                    });
                    strip.strip(|builder| {
                        builder
                            .size(Size::relative(0.5))
                            .size(Size::relative(0.5))
                            .horizontal(|mut strip| {
                                strip.cell(|ui| {
                                    let (_response, painter) =
                                        ui.allocate_painter(Vec2::new(ui.available_width(), 300.0), Sense::hover());
                                    painter.add(egui::Shape::line_segment(
                                        [egui::Pos2::new(0.0, 0.0), egui::Pos2::new(100.0, 100.0)],
                                        (1.0, egui::Color32::WHITE),
                                    ));
                                });
                                strip.cell(|ui| {
                                    CodeEditor::default()
                                        .id_source("code editor2")
                                        .with_rows(12)
                                        .with_fontsize(14.0)
                                        .with_theme(ColorTheme::SONOKAI)
                                        .with_syntax(vec_op_syntax())
                                        .with_numlines(true)
                                        .show(ui, &mut self.code);
                                })
                            });
                    });
                });
        });
    }
}
