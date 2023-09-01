use std::ops::RangeInclusive;
use eframe::egui;
use eframe::egui::plot::{Line, LineStyle, Plot};
use eframe::egui::{Stroke, Ui};
use crate::COLOR_PALETTE;
use crate::common_vec_op::VecLineData;
use crate::interfaces::{IVisData, IVisualizer};

pub struct CommonVecVisualizer {
    t: [[f64; 3]; 3],
}

impl IVisualizer for CommonVecVisualizer {
    type VDT = VecLineData;

    fn new(transform: [[f64; 3]; 3]) -> Self {
        Self {
            t: transform,
        }
    }

    fn plot(&self, ui: &mut Ui, input: Vec<Vec<Self::VDT>>, has_error: bool, show_inter_dash: bool, colorful_block: bool) {
        let (a, b,
            c, d) = (
            self.t[0][0], self.t[1][1],
            self.t[0][2], self.t[1][2]);

        let plot = Plot::new("plot").data_aspect(1.0)
            .x_axis_formatter(
                move |x: f64, _range: &RangeInclusive<f64>| format!("{:.0}", a * x + c)
            )
            .y_axis_formatter(
                move |y: f64, _range: &RangeInclusive<f64>| format!("{:.0}", b * y + d)
            );
        plot.show(ui, |plot_ui| {
            let lines = input;
            if lines.len() == 0 {
                return;
            }
            let mut last_line_end = lines.first().unwrap().last().unwrap().clone();
            let mut color_index = 0;
            for points in lines.into_iter() {
                let mut points = points;
                points = points.into_iter().map(|v| v.matrix(self.t)).collect::<Vec<VecLineData>>();
                let curr_line_start = points.first().unwrap().clone();
                if last_line_end != curr_line_start && show_inter_dash {
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

                if colorful_block {
                    color_index = (color_index + 1) % COLOR_PALETTE.len();
                }
            }
        });
    }

    fn transform(&mut self, matrix: [[f64; 3]; 3]) {
        self.t = matrix;
    }
}