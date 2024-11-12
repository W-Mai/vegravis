use crate::common_vec_op::VecLineData;
use crate::interfaces::{IVisData, IVisualizer};
use crate::COLOR_PALETTE;
use eframe::egui;
use eframe::egui::Stroke;
use egui_plot::{Line, LineStyle, Plot};
use std::ops::Neg;

pub struct CommonVecVisualizer {
    t: [[f64; 3]; 3],
}

impl IVisualizer for CommonVecVisualizer {
    fn new(transform: [[f64; 3]; 3]) -> Self {
        Self { t: transform }
    }

    fn plot(
        &self,
        ui: &mut egui::Ui,
        input: Vec<Vec<Box<dyn IVisData>>>,
        has_error: bool,
        show_inter_dash: bool,
        colorful_block: bool,
        lcd_coords: bool,
    ) {
        let mut trans_matrix = self.t;
        if lcd_coords {
            trans_matrix[0][1] = trans_matrix[0][1].neg();
            trans_matrix[1][1] = trans_matrix[1][1].neg();
            trans_matrix[2][1] = trans_matrix[2][1].neg();
        }
        let plot = Plot::new("plot")
            .data_aspect(1.0)
            .y_axis_formatter(move |mk, _range| {
                format!("{:.0}", if lcd_coords { mk.value.neg() } else { mk.value })
            });
        plot.show(ui, |plot_ui| {
            let lines = input;
            if lines.is_empty() {
                return;
            }
            let mut last_line_end = lines.first().unwrap().last().unwrap().clone();
            let mut color_index = 0;
            for points in lines.into_iter() {
                if points.is_empty() {
                    continue;
                }
                let points = points
                    .into_iter()
                    .map(|v| v.matrix(trans_matrix).cast())
                    .collect::<Vec<VecLineData>>();
                let curr_line_start = *points.first().unwrap();
                if !last_line_end.is_same(&curr_line_start as &dyn IVisData) && show_inter_dash {
                    let last_line_end_pos: [f64; 2] = [
                        *last_line_end.pos()[0].cast_ref(),
                        *last_line_end.pos()[1].cast_ref(),
                    ];
                    let curr_line_start_pos: [f64; 2] = [
                        *curr_line_start.pos()[0].cast_ref(),
                        *curr_line_start.pos()[1].cast_ref(),
                    ];
                    let drawn_lines = Line::new(vec![last_line_end_pos, curr_line_start_pos])
                        .style(LineStyle::dashed_dense());
                    plot_ui.line(if has_error {
                        drawn_lines.stroke(Stroke::new(2.0, egui::Color32::LIGHT_RED))
                    } else {
                        drawn_lines.stroke(Stroke::new(1.0, egui::Color32::LIGHT_GREEN))
                    });
                }
                last_line_end = Box::new(*points.last().unwrap());
                let points: Vec<[f64; 2]> = points
                    .into_iter()
                    .map(|v| [*v.pos()[0].cast_ref(), *v.pos()[1].cast_ref()])
                    .collect();
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
