/// Stds
use std::rc::Rc;

/// Crates
use crate::any_data::AnyData;
use crate::interfaces::ICommandDescription;

/// Self
use super::{calc_trans_stack, GenerateCtx};

pub struct CommonOpPushTrans;

pub struct CommonOpPushWorldTrans;

impl ICommandDescription for CommonOpPushTrans {
    fn name(&self) -> Vec<&str> {
        ["PUSH_TRANS"].into()
    }

    fn argc(&self) -> usize {
        9
    }

    fn operate(&self, ctx: &mut AnyData, argv: Rc<Vec<AnyData>>) -> Vec<AnyData> {
        let ctx = ctx.cast_mut::<GenerateCtx>();
        let trans_matrix: [[f64; 3]; 3] = [
            [
                *argv[0].cast_ref(),
                *argv[1].cast_ref(),
                *argv[2].cast_ref(),
            ],
            [
                *argv[3].cast_ref(),
                *argv[4].cast_ref(),
                *argv[5].cast_ref(),
            ],
            [
                *argv[6].cast_ref(),
                *argv[7].cast_ref(),
                *argv[8].cast_ref(),
            ],
        ];

        ctx.local_trans_stack.push(trans_matrix);
        ctx.current_local_trans = calc_trans_stack(&ctx.local_trans_stack);

        vec![]
    }
}

impl ICommandDescription for CommonOpPushWorldTrans {
    fn name(&self) -> Vec<&str> {
        ["PUSH_WORLD_TRANS"].into()
    }
    fn argc(&self) -> usize {
        9
    }

    fn operate(&self, ctx: &mut AnyData, argv: Rc<Vec<AnyData>>) -> Vec<AnyData> {
        let ctx = ctx.cast_mut::<GenerateCtx>();
        let trans_matrix: [[f64; 3]; 3] = [
            [
                *argv[0].cast_ref(),
                *argv[1].cast_ref(),
                *argv[2].cast_ref(),
            ],
            [
                *argv[3].cast_ref(),
                *argv[4].cast_ref(),
                *argv[5].cast_ref(),
            ],
            [
                *argv[6].cast_ref(),
                *argv[7].cast_ref(),
                *argv[8].cast_ref(),
            ],
        ];

        ctx.world_trans_stack.push(trans_matrix);
        ctx.current_world_trans = calc_trans_stack(&ctx.world_trans_stack);

        vec![]
    }
}
