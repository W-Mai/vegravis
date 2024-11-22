/// Stds
use std::rc::Rc;

/// Crates
use crate::any_data::AnyData;
use crate::interfaces::ICommandDescription;

/// Self
use super::{calc_trans_stack, GenerateCtx};

pub struct CommonOpPushSkew;

pub struct CommonOpPushWorldSkew;

impl ICommandDescription for CommonOpPushSkew {
    fn name(&self) -> Vec<&str> {
        ["PUSH_SKEW", "SKEW"].into()
    }

    fn argc(&self) -> usize {
        2
    }

    fn operate(&self, ctx: &mut AnyData, argv: Rc<Vec<AnyData>>) -> Vec<AnyData> {
        let ctx = ctx.cast_mut::<GenerateCtx>();
        let trans_matrix: [[f64; 3]; 3] = [
            [1.0, *argv[0].cast_ref(), 0.0],
            [*argv[1].cast_ref(), 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];

        ctx.local_trans_stack.push(trans_matrix);
        ctx.current_local_trans = calc_trans_stack(&ctx.local_trans_stack);

        vec![]
    }
}

impl ICommandDescription for CommonOpPushWorldSkew {
    fn name(&self) -> Vec<&str> {
        ["PUSH_WORLD_SKEW", "WORLD_SKEW"].into()
    }

    fn argc(&self) -> usize {
        2
    }

    fn operate(&self, ctx: &mut AnyData, argv: Rc<Vec<AnyData>>) -> Vec<AnyData> {
        let ctx = ctx.cast_mut::<GenerateCtx>();
        let trans_matrix: [[f64; 3]; 3] = [
            [1.0, *argv[0].cast_ref(), 0.0],
            [*argv[1].cast_ref(), 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];

        ctx.world_trans_stack.push(trans_matrix);
        ctx.current_world_trans = calc_trans_stack(&ctx.world_trans_stack);

        vec![]
    }
}
