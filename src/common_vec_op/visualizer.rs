use std::ops::RangeInclusive;
use eframe::egui::plot::Plot;
use crate::interfaces::IVisualizer;

pub struct CommonVecVisualizer {
    p: Plot
}

// impl IVisualizer<PT, VDT> for CommonVecVisualizer {
//     fn plot(&self, input: VDT) {
//         let plot = Plot::new("plot").data_aspect(1.0)
//             .y_axis_formatter(
//                 if self.params.lcd_coords {
//                     |y: f64, _range: &RangeInclusive<f64>| format!("{:.0}", -y)
//                 } else {
//                     |y: f64, _range: &RangeInclusive<f64>| format!("{:.0}", y)
//                 }
//             );
//     }
// }
