#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod syntax;

use std::f64::consts::TAU;
use eframe::{egui};
use eframe::egui::{remap};
use eframe::egui::plot::{Line, Plot, PlotPoint, PlotPoints};
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
                                    let plot = Plot::new("plot").data_aspect(1.0);
                                    plot.show(ui, |plot_ui| {
                                        let n = 512;
                                        let circle_points: PlotPoints = (0..=n)
                                            .map(|i| {
                                                let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
                                                let r = 10.0;
                                                [
                                                    r * t.cos() + 10.0f64,
                                                    r * t.sin() + 10.0f64,
                                                ]
                                            })
                                            .collect();
                                        plot_ui.line(Line::new(circle_points).color(egui::Color32::RED));
                                        plot_ui.line(Line::new(MainApp::gen_line(PlotPoint::from([10.0, 10.0]), PlotPoint::from([10.0, 300.0]))).color(egui::Color32::GREEN));
                                        plot_ui.line(Line::new(MainApp::gen_line(PlotPoint::from([10.0, 300.0]), PlotPoint::from([300.0, 300.0]))).color(egui::Color32::GREEN));
                                        plot_ui.line(Line::new(MainApp::gen_line(PlotPoint::from([300.0, 300.0]), PlotPoint::from([300.0, 10.0]))).color(egui::Color32::GREEN));
                                        plot_ui.line(Line::new(MainApp::gen_line(PlotPoint::from([300.0, 10.0]), PlotPoint::from([10.0, 10.0]))).color(egui::Color32::GREEN));
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

impl MainApp {
    fn gen_line(start: PlotPoint, end: PlotPoint) -> PlotPoints {
        let n = 512;
        let points: PlotPoints = (0..=n).map(|i| {
            let t = remap(i as f64, 0.0..=(n as f64), 0.0..=1.0);
            let x = start.x + (end.x - start.x) * t;
            let y = start.y + (end.y - start.y) * t;
            [x, y]
        }).collect();

        points
    }
}
