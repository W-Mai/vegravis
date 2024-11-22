/// Stds
use std::rc::Rc;

/// Crates
use crate::any_data::AnyData;
use crate::interfaces::ICommandDescription;

/// Self
use super::{calc_trans_stack, GenerateCtx};

pub struct CommonOpPushRotate;

impl ICommandDescription for CommonOpPushRotate {
    fn name(&self) -> Vec<&str> {
        ["PUSH_ROTATE", "ROTATE"].into()
    }

    fn argc(&self) -> usize {
        1
    }

    fn operate(&self, ctx: &mut AnyData, argv: Rc<Vec<AnyData>>) -> Vec<AnyData> {
        let ctx = ctx.cast_mut::<GenerateCtx>();

        let angle: f64 = *argv[0].cast_ref();
        let angle_cos = angle.cos();
        let angle_sin = angle.sin();

        let trans_matrix: [[f64; 3]; 3] = [
            [angle_cos, -angle_sin, 0.0],
            [angle_sin, angle_cos, 0.0],
            [0.0, 0.0, 1.0],
        ];

        ctx.local_trans_stack.push(trans_matrix);
        ctx.current_local_trans = calc_trans_stack(&ctx.local_trans_stack);

        vec![]
    }
}
