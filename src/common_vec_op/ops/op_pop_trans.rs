/// Stds
use std::rc::Rc;

/// Crates
use crate::any_data::AnyData;
use crate::interfaces::ICommandDescription;

/// Self
use super::{calc_trans_stack, GenerateCtx};

pub struct CommonOpPopTrans;

impl ICommandDescription for CommonOpPopTrans {
    fn name(&self) -> Vec<&str> {
        ["POP_TRANS"].into()
    }

    fn argc(&self) -> usize {
        0
    }

    fn operate(&self, ctx: &mut AnyData, _argv: Rc<Vec<AnyData>>) -> Vec<AnyData> {
        let ctx = ctx.cast_mut::<GenerateCtx>();
        if ctx.local_trans.is_empty() {
            return vec![];
        }
        ctx.local_trans.pop();
        ctx.current_trans = calc_trans_stack(&ctx.local_trans);

        vec![]
    }
}
