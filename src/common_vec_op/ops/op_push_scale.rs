/// Stds
use std::rc::Rc;

/// Crates
use crate::any_data::AnyData;
use crate::interfaces::ICommandDescription;

/// Self
use super::{calc_trans_stack, GenerateCtx};

pub struct CommonOpPushScale;

impl ICommandDescription for CommonOpPushScale {
    fn name(&self) -> Vec<&str> {
        ["PUSH_SCALE", "SCALE"].into()
    }

    fn argc(&self) -> usize {
        2
    }

    fn operate(&self, ctx: &mut AnyData, argv: Rc<Vec<AnyData>>) -> Vec<AnyData> {
        let ctx = ctx.cast_mut::<GenerateCtx>();
        let trans_matrix: [[f64; 3]; 3] = [
            [*argv[0].cast_ref(), 0.0, 0.0],
            [0.0, *argv[1].cast_ref(), 0.0],
            [0.0, 0.0, 1.0],
        ];

        ctx.local_trans_stack.push(trans_matrix);
        ctx.current_local_trans = calc_trans_stack(&ctx.local_trans_stack);

        vec![]
    }
}
