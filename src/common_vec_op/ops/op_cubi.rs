/// Stds
use std::rc::Rc;

/// 3rds
use egui_plot::PlotPoint;

/// Crates
use crate::any_data::AnyData;
use crate::interfaces::ICommandDescription;

/// Self
use super::{GenerateCtx, VecLineData, process_point};

pub struct CommonOpCUBI;

impl ICommandDescription for CommonOpCUBI {
    fn name(&self) -> Vec<&str> {
        ["CUBI", "CUBIC"].into()
    }

    fn argc(&self) -> usize {
        6
    }

    fn operate(&self, ctx: &mut AnyData, argv: Rc<Vec<AnyData>>) -> Vec<AnyData> {
        let ctx = ctx.cast_mut::<GenerateCtx>();
        let current_matrix = ctx.current_local_trans;

        let argv = process_point(argv, current_matrix);

        let [x1, y1, x2, y2, x3, y3] = [argv[0], argv[1], argv[2], argv[3], argv[4], argv[5]];

        let cursor = ctx.cursor;
        let mut points = Vec::new();
        let mut t = 0.0;
        while t < 1.0 {
            let x = (1.0f64 - t).powi(3) * cursor.x
                + 3.0 * (1.0 - t).powi(2) * t * x1
                + 3.0 * (1.0 - t) * t.powi(2) * x2
                + t.powi(3) * x3;
            let y = (1.0f64 - t).powi(3) * cursor.y
                + 3.0 * (1.0 - t).powi(2) * t * y1
                + 3.0 * (1.0 - t) * t.powi(2) * y2
                + t.powi(3) * y3;
            points.push(VecLineData::new(x, y));
            t += 0.01;
        }

        ctx.grouping = true;
        ctx.cursor = PlotPoint::from([x3, y3]);

        AnyData::convert_to_vec(points)
    }
}
