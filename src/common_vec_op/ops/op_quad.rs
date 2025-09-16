/// Stds
use std::rc::Rc;

/// 3rds
use egui_plot::PlotPoint;

/// Crates
use crate::any_data::AnyData;
use crate::interfaces::ICommandDescription;

/// Self
use super::{GenerateCtx, VecLineData, process_point};

pub struct CommonOpQUAD;

impl ICommandDescription for CommonOpQUAD {
    fn name(&self) -> Vec<&str> {
        ["QUAD"].into()
    }

    fn argc(&self) -> usize {
        4
    }

    fn operate(&self, ctx: &mut AnyData, argv: Rc<Vec<AnyData>>) -> Vec<AnyData> {
        let ctx = ctx.cast_mut::<GenerateCtx>();
        let current_matrix = ctx.current_local_trans;

        let argv = process_point(argv, current_matrix);

        let [x1, y1, x2, y2] = [argv[0], argv[1], argv[2], argv[3]];

        let cursor = ctx.cursor;
        let mut points = Vec::new();
        let mut t = 0.0;
        while t < 1.0 {
            let x = (1.0f64 - t).powi(2) * cursor.x + 2.0 * (1.0 - t) * t * x1 + t.powi(2) * x2;
            let y = (1.0f64 - t).powi(2) * cursor.y + 2.0 * (1.0 - t) * t * y1 + t.powi(2) * y2;
            points.push(VecLineData::new(x, y));
            t += 0.01;
        }

        ctx.grouping = true;
        ctx.cursor = PlotPoint::from([x2, y2]);

        AnyData::convert_to_vec(points)
    }
}
