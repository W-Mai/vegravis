#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod syntax;
mod common_vec_op;
mod cus_component;
mod interfaces;

use std::ops::RangeInclusive;
use std::vec;
use log::error;
use eframe::{egui};
use eframe::egui::plot::{Line, LineStyle, Plot};
use eframe::egui::Stroke;
use egui_code_editor::{CodeEditor, ColorTheme};
use egui_extras::{Size, StripBuilder};
use crate::common_vec_op::{CodeParser, TextDataSrc, VecLineData, VecLineGen};
use crate::interfaces::{ICommandSyntax, IDataSource, IParser, IVisData, IVisDataGenerator, ParseError};
use crate::cus_component::toggle;
use crate::syntax::{CommonVecOpSyntax};

const DEFAULT_CODE: &str = include_str!("default_code");

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
    lines: Vec<Vec<VecLineData>>,

    params: MainAppParams,
}

#[derive(Clone, PartialEq, Default)]
struct MainAppParams {
    vis_progress: i64,
    vis_progress_max: i64,

    lcd_coords: bool,
    show_inter_dash: bool,
    colorful_block: bool,

    trans_matrix: [[f64; 3]; 3],
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
            params: MainAppParams::default(),
            cache: MainAppCache {
                code: "".to_owned(),
                lines: vec![],
                params: MainAppParams::default(),
            },
            error: None,
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.params.trans_matrix = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ]; // Identity matrix

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
                                    if self.params.lcd_coords {
                                        self.params.trans_matrix[1][1] = -1.0;
                                    }
                                    let (a, b,
                                        c, d) = (
                                        self.params.trans_matrix[0][0], self.params.trans_matrix[1][1],
                                        self.params.trans_matrix[0][2], self.params.trans_matrix[1][2]);
                                    let plot = Plot::new("plot").data_aspect(1.0)
                                        .x_axis_formatter(
                                            move |x: f64, _range: &RangeInclusive<f64>| format!("{:.0}", a * x + c)
                                        )
                                        .y_axis_formatter(
                                            move |y: f64, _range: &RangeInclusive<f64>| format!("{:.0}", b * y + d)
                                        );

                                    let mut has_error = false;
                                    plot.show(ui, |plot_ui| {
                                        if self.code != self.cache.code || self.params != self.cache.params {
                                            let mut parser = CodeParser::new(TextDataSrc::new(self.code.clone()), VecLineGen::default());

                                            has_error = match parser.parse() {
                                                Ok(vlg) => {
                                                    let ops_count = vlg.len() as i64;
                                                    self.params.vis_progress_max = ops_count;
                                                    if self.code != self.cache.code {
                                                        self.params.vis_progress = ops_count;
                                                    }

                                                    let parsed = vlg.gen(0..(self.params.vis_progress));

                                                    self.cache.lines = parsed.clone();
                                                    self.cache.code = self.code.clone();
                                                    self.cache.params = self.params.clone();
                                                    false
                                                }
                                                Err(e) => {
                                                    error!("Error: {:?}", e);
                                                    self.error = Some(e);
                                                    true
                                                }
                                            }
                                        }

                                        if !has_error {
                                            self.error = None;
                                        }
                                        let lines = self.cache.lines.clone();
                                        if lines.len() == 0 {
                                            return;
                                        }
                                        let mut last_line_end = lines.first().unwrap().last().unwrap().clone();
                                        let mut color_index = 0;
                                        for points in lines.into_iter() {
                                            let mut points = points;
                                            if self.params.lcd_coords {
                                                points = points.into_iter().map(|v| v.matrix(self.params.trans_matrix)).collect::<Vec<VecLineData>>();
                                            }
                                            let curr_line_start = points.first().unwrap().clone();
                                            if last_line_end != curr_line_start && self.params.show_inter_dash {
                                                let drawn_lines = Line::new(vec![last_line_end.pos(), curr_line_start.pos()])
                                                    .style(LineStyle::dashed_dense());
                                                plot_ui.line(if has_error {
                                                    drawn_lines.stroke(Stroke::new(2.0, egui::Color32::LIGHT_RED))
                                                } else {
                                                    drawn_lines.stroke(Stroke::new(1.0, egui::Color32::LIGHT_GREEN))
                                                });
                                            }
                                            last_line_end = points.last().unwrap().clone();
                                            let points: Vec<[f64; 2]> = points.into_iter().map(|v| v.pos()).collect();
                                            let drawn_lines = Line::new(points);
                                            plot_ui.line(if has_error {
                                                drawn_lines.color(egui::Color32::DARK_RED).width(5.0)
                                            } else {
                                                drawn_lines.color(COLOR_PALETTE[color_index]).width(2.0)
                                            });

                                            if self.params.colorful_block {
                                                color_index = (color_index + 1) % COLOR_PALETTE.len();
                                            }
                                        }
                                    });
                                });
                                strip.strip(|builder| {
                                    builder
                                        .size(Size::exact(60.0))
                                        .size(Size::exact(30.0))
                                        .size(Size::remainder())
                                        .vertical(|mut strip| {
                                            strip.cell(|ui| {
                                                egui::ScrollArea::vertical().show(ui, |ui| {
                                                    ui.vertical_centered(|ui| {
                                                        ui.heading("Controls");
                                                    });
                                                    ui.horizontal_wrapped(|ui| {
                                                        ui.add(toggle("LCD Coordinates", &mut self.params.lcd_coords));
                                                        ui.add(toggle("Show Intermediate Dash", &mut self.params.show_inter_dash));
                                                        ui.add(toggle("Colorful Blocks", &mut self.params.colorful_block));
                                                        ui.add_sized(ui.available_size(),
                                                                     egui::Slider::new(&mut self.params.vis_progress, 0..=self.params.vis_progress_max)
                                                                         .text("Progress")
                                                                         .show_value(true),
                                                        );
                                                    });
                                                });
                                            });
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
                                                    .with_syntax(CommonVecOpSyntax {}.syntax())
                                                    .with_numlines(true)
                                                    .show(ui, &mut self.code);
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
