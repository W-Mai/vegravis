/// Stds
use std::rc::Rc;

/// 3rds
use egui_plot::PlotPoint;

/// Crates
use crate::any_data::AnyData;
use crate::interfaces::ICommandDescription;

/// Self
use super::{process_point, GenerateCtx, VecLineData};

pub struct CommonOpMOVE;

impl ICommandDescription for CommonOpMOVE {
    fn name(&self) -> Vec<&str> {
        ["MOVE"].into()
    }

    fn argc(&self) -> usize {
        2
    }

    fn operate(&self, ctx: &mut AnyData, argv: Rc<Vec<AnyData>>) -> Vec<AnyData> {
        let ctx = ctx.cast_mut::<GenerateCtx>();

        let current_matrix = ctx.current_local_trans;

        let argv = process_point(argv, current_matrix);
        let nums = [argv[0], argv[1]];

        let points = vec![VecLineData::new(nums[0], nums[1])];

        ctx.grouping = false;
        ctx.cursor = PlotPoint::from(nums);

        AnyData::convert_to_vec(points)
    }
}
