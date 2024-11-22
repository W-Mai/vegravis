/// Stds
use std::rc::Rc;

/// Crates
use crate::any_data::AnyData;
use crate::interfaces::ICommandDescription;

pub struct CommonOpEND;

impl ICommandDescription for CommonOpEND {
    fn name(&self) -> Vec<&str> {
        ["END", "CLOSE"].into()
    }

    fn argc(&self) -> usize {
        0
    }

    fn operate(&self, _ctx: &mut AnyData, _argv: Rc<Vec<AnyData>>) -> Vec<AnyData> {
        vec![]
    }
}
