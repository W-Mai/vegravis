#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod syntax;
mod vec_line_gen;
mod code_parser;

use std::vec;
use log::error;
use eframe::{egui};
use eframe::egui::plot::{Line, Plot, PlotPoints};
use egui_code_editor::{CodeEditor, ColorTheme};
use egui_extras::{Size, StripBuilder};
use crate::code_parser::ParseError;
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
    lines: Vec<Vec<[f64; 2]>>,

    params: MainAppParams,
}

#[derive(Clone, PartialEq)]
struct MainAppParams {
    vis_progress: i64,
    vis_progress_max: i64,
}

struct MainApp {
    code: String,
    error: Option<ParseError>,

    params: MainAppParams,

    cache: MainAppCache,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            code: DEFAULT_CODE.to_owned(),
            params: MainAppParams {
                vis_progress: 0,
                vis_progress_max: 0,
            },
            cache: MainAppCache {
                code: "".to_owned(),
                lines: vec![],
                params: MainAppParams {
                    vis_progress: 0,
                    vis_progress_max: 0,
                },
            },
            error: None,
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::exact(30.0))
                .size(Size::remainder())
                .size(Size::exact(30.0))
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
                                        if self.code != self.cache.code || self.params != self.cache.params {
                                            let mut parser = code_parser::CodeParser::new(self.code.clone());

                                            match parser.parse() {
                                                Ok(parsed) => {
                                                    let ops_count = parsed.len() as i64;
                                                    self.params.vis_progress_max = ops_count;
                                                    if self.code != self.cache.code {
                                                        self.params.vis_progress = ops_count;
                                                    }

                                                    let showed_ops = parsed.split_at(self.params.vis_progress as usize).0.clone();
                                                    let showed_ops = showed_ops.to_vec();
                                                    let mut vlg = VecLineGen::new(showed_ops);

                                                    self.cache.lines = vlg.gen();
                                                    self.cache.code = self.code.clone();
                                                    self.cache.params = self.params.clone();
                                                }
                                                Err(e) => {
                                                    error!("Error: {:?}", e);
                                                    self.error = Some(e);
                                                    let lines = self.cache.lines.clone();
                                                    for points in lines.into_iter() {
                                                        plot_ui.line(Line::new(points).color(egui::Color32::DARK_RED).width(5.0));
                                                    }
                                                    return;
                                                }
                                            }
                                        }
                                        self.error = None;
                                        let lines = self.cache.lines.clone();
                                        for points in lines.into_iter() {
                                            plot_ui.line(Line::new(points).color(egui::Color32::DARK_BLUE).width(5.0));
                                        }
                                    });
                                });
                                strip.strip(|builder| {
                                    builder
                                        .size(Size::exact(30.0))
                                        .size(Size::remainder())
                                        .size(Size::exact(30.0))
                                        .vertical(|mut strip| {
                                            strip.cell(|ui| {
                                                ui.vertical_centered(|ui| {
                                                    ui.heading("Code");
                                                });
                                            });
                                            strip.cell(|ui| {
                                                CodeEditor::default()
                                                    .id_source("code editor")
                                                    .with_rows(12)
                                                    .with_fontsize(14.0)
                                                    .with_theme(ColorTheme::SONOKAI)
                                                    .with_syntax(vec_op_syntax())
                                                    .with_numlines(true)
                                                    .show(ui, &mut self.code);
                                            });

                                            strip.cell(|ui| {
                                                ui.add_sized(ui.available_size(),
                                                             egui::Slider::new(&mut self.params.vis_progress, 0..=self.params.vis_progress_max)
                                                                 .text("Progress")
                                                                 .show_value(true),
                                                );
                                            });
                                        });
                                })
                            });
                    });
                    strip.cell(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.horizontal(|ui| {
                                let info = self.error.as_ref().map_or_else(|| "".to_owned(), |e| {
                                    format!("({}, {}): Error: {}", e.cursor.row + 1, e.cursor.col, e.msg)
                                });
                                let rt = egui::RichText::new(info).size(20.0).color(egui::Color32::RED).text_style(egui::TextStyle::Monospace);
                                ui.label(rt).highlight();
                            });
                        });
                    });
                });
        });
    }
}
