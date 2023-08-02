#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod syntax;
mod vec_line_gen;
mod code_parser;

use eframe::{egui};
use eframe::egui::plot::{Line, Plot, PlotPoints};
use egui_code_editor::{CodeEditor, ColorTheme};
use egui_extras::{Size, StripBuilder};
use crate::syntax::vec_op_syntax;
use crate::vec_line_gen::VecLineGen;

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

struct MainAppCache {
    code: String,
    lines: Vec<[f64; 2]>,
}

struct MainApp {
    code: String,
    cache: MainAppCache,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            code: DEFAULT_CODE.to_owned(),
            cache: MainAppCache {
                code: "".to_owned(),
                lines: vec![],
            },
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
                                    let plot = Plot::new("plot").data_aspect(1.0);
                                    plot.show(ui, |plot_ui| {
                                        if self.code != self.cache.code {
                                            let mut parser = code_parser::CodeParser::new(self.code.clone());
                                            let mut vlg = VecLineGen::new(parser.parse());
                                            self.cache.lines = vlg.gen();
                                            self.cache.code = self.code.clone();
                                        }
                                        let lines = self.cache.lines.clone();
                                        let points: PlotPoints = lines.into_iter().collect();
                                        plot_ui.line(Line::new(points).color(egui::Color32::from_rgb(0, 255, 0)));
                                    });
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
