#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod common_vec_op;
mod cus_component;
mod interfaces;
mod any_data;

use std::vec;
use log::error;
use eframe::{egui};
use egui_extras::{Size, StripBuilder};
use crate::any_data::AnyData;
use crate::common_vec_op::{CodeParser, CommonVecVisualizer, VecLineGen};
use crate::interfaces::{ICodeEditor, IParser, IVisData, IVisDataGenerator, IVisualizer, ParseError};
use crate::cus_component::{CodeEditor, toggle};

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
        initial_window_size: Some(egui::vec2(1440.0, 960.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Vector Graphics Visualizer",
        options,
        Box::new(|_cc| Box::<MainApp>::default()),
    )
}

struct MainAppCache {
    code: AnyData,
    lines: Vec<Vec<Box<dyn IVisData>>>,

    params: MainAppParams,
}

#[derive(Clone, PartialEq)]
struct MainAppParams {
    vis_progress: i64,
    vis_progress_max: i64,

    lcd_coords: bool,
    show_inter_dash: bool,
    colorful_block: bool,

    trans_matrix: [[f64; 3]; 3],
}

impl Default for MainAppParams {
    fn default() -> Self {
        Self {
            vis_progress: 0,
            vis_progress_max: 0,
            lcd_coords: false,
            show_inter_dash: false,
            colorful_block: false,
            trans_matrix: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ], // Identity matrix
        }
    }
}

struct MainApp {
    code: AnyData,
    error: Option<ParseError>,

    params: MainAppParams,

    cache: MainAppCache,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            code: AnyData::new(DEFAULT_CODE.to_owned()),
            params: MainAppParams::default(),
            cache: MainAppCache {
                code: AnyData::new("".to_owned()),
                lines: vec![],
                params: MainAppParams::default(),
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
                                    self.ui_visualizer(ui);
                                });
                                strip.strip(|builder| {
                                    builder
                                        .size(Size::exact(180.0))
                                        .size(Size::exact(30.0))
                                        .size(Size::remainder())
                                        .vertical(|mut strip| {
                                            strip.cell(|ui| {
                                                self.ui_options_panel(ui);
                                            });
                                            strip.cell(|ui| {
                                                ui.vertical_centered(|ui| {
                                                    ui.heading("Code");
                                                });
                                            });
                                            strip.cell(|ui| {
                                                self.ui_code_editor(ui);
                                            });
                                        });
                                })
                            });
                    });
                    strip.cell(|ui| {
                        self.ui_toast_bar(ui);
                    });
                });
        });
    }
}

impl MainApp {
    fn ui_toast_bar(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                let info = self.error.as_ref().map_or_else(|| "".to_owned(), |e| {
                    format!("({}, {}): Error: {}", e.cursor.row + 1, e.cursor.col, e.msg)
                });
                let rt = egui::RichText::new(info).size(20.0).color(egui::Color32::RED).text_style(egui::TextStyle::Monospace);
                ui.label(rt).highlight();
            });
        });
    }

    fn ui_options_panel(&mut self, ui: &mut egui::Ui) {
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
            StripBuilder::new(ui)
                .size(Size::exact(30.0))
                .size(Size::remainder())
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Transform Matrix");
                        });
                    });
                    strip.strip(|builder| {
                        builder
                            .size(Size::exact(20.0))
                            .size(Size::exact(20.0))
                            .size(Size::exact(20.0)) // 3x3 matrix
                            .vertical(|mut strip| {
                                strip.strip(|builder| {
                                    builder
                                        .size(Size::relative(0.33))
                                        .size(Size::relative(0.33))
                                        .size(Size::relative(0.33)) // 3x3 matrix
                                        .horizontal(|mut strip| {
                                            strip.cell(|ui| {
                                                ui.add_sized(ui.available_size(),
                                                             egui::Slider::new(&mut self.params.trans_matrix[0][0], -5.0..=5.0)
                                                                 .text("m00")
                                                                 .show_value(true),
                                                );
                                            });
                                            strip.cell(|ui| {
                                                ui.add_sized(ui.available_size(),
                                                             egui::Slider::new(&mut self.params.trans_matrix[0][1], -5.0..=5.0)
                                                                 .text("m01")
                                                                 .show_value(true),
                                                );
                                            });
                                            strip.cell(|ui| {
                                                ui.add_sized(ui.available_size(),
                                                             egui::Slider::new(&mut self.params.trans_matrix[0][2], -100.0..=100.0)
                                                                 .text("m02")
                                                                 .show_value(true),
                                                );
                                            });
                                        });
                                });
                                strip.strip(|builder| {
                                    builder
                                        .size(Size::relative(0.33))
                                        .size(Size::relative(0.33))
                                        .size(Size::relative(0.33)) // 3x3 matrix
                                        .horizontal(|mut strip| {
                                            strip.cell(|ui| {
                                                ui.add_sized(ui.available_size(),
                                                             egui::Slider::new(&mut self.params.trans_matrix[1][0], -5.0..=5.0)
                                                                 .text("m10")
                                                                 .show_value(true),
                                                );
                                            });
                                            strip.cell(|ui| {
                                                ui.add_sized(ui.available_size(),
                                                             egui::Slider::new(&mut self.params.trans_matrix[1][1], -5.0..=5.0)
                                                                 .text("m11")
                                                                 .show_value(true),
                                                );
                                            });
                                            strip.cell(|ui| {
                                                ui.add_sized(ui.available_size(),
                                                             egui::Slider::new(&mut self.params.trans_matrix[1][2], -100.0..=100.0)
                                                                 .text("m12")
                                                                 .show_value(true),
                                                );
                                            });
                                        });
                                });
                                strip.strip(|builder| {
                                    builder
                                        .size(Size::relative(0.33))
                                        .size(Size::relative(0.33))
                                        .size(Size::relative(0.33)) // 3x3 matrix
                                        .horizontal(|mut strip| {
                                            strip.cell(|ui| {
                                                ui.add_sized(ui.available_size(),
                                                             egui::Slider::new(&mut self.params.trans_matrix[2][0], -5.0..=5.0)
                                                                 .text("m20")
                                                                 .show_value(true),
                                                );
                                            });
                                            strip.cell(|ui| {
                                                ui.add_sized(ui.available_size(),
                                                             egui::Slider::new(&mut self.params.trans_matrix[2][1], -5.0..=5.0)
                                                                 .text("m21")
                                                                 .show_value(true),
                                                );
                                            });
                                            strip.cell(|ui| {
                                                ui.add_sized(ui.available_size(),
                                                             egui::Slider::new(&mut self.params.trans_matrix[2][2], -5.0..=5.0)
                                                                 .text("m22")
                                                                 .show_value(true),
                                                );
                                            });
                                        });
                                });
                            });
                    });
                });
        });
    }

    fn ui_code_editor(&mut self, ui: &mut egui::Ui) {
        CodeEditor {}.show(ui, &mut self.code, VecLineGen::default().command_syntax());
    }

    fn ui_visualizer(&mut self, ui: &mut egui::Ui) {
        if self.params.lcd_coords {
            self.params.trans_matrix[1][1] = -1.0;
        }
        let visualizer = CommonVecVisualizer::new(self.params.trans_matrix);

        let mut has_error = false;
        if !self.code.equal::<String, String>(&self.cache.code) || self.params != self.cache.params {
            let mut generator = VecLineGen::default();
            let mut parser = CodeParser::new(self.code.clone::<String>(), &mut generator);
            // 通过parser产生generator需要的前置数据
            has_error = match parser.parse() {
                Ok(vlg) => {
                    let ops_count = vlg.len() as i64;
                    self.params.vis_progress_max = ops_count;
                    if !self.code.equal::<String, String>(&self.cache.code) {
                        self.params.vis_progress = ops_count;
                    }

                    let parsed = vlg.gen(0..(self.params.vis_progress));

                    self.cache.lines = parsed.clone();
                    self.cache.code = self.code.clone::<String>();
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

        visualizer.plot(
            ui,
            self.cache.lines.clone(),
            has_error,
            self.params.show_inter_dash,
            self.params.colorful_block,
        );
    }
}
