pub(crate) mod op_cubi;
pub(crate) mod op_end;
pub(crate) mod op_line;
pub(crate) mod op_move;
pub(crate) mod op_pop_trans;
pub(crate) mod op_push_rotate;
pub(crate) mod op_push_scale;
pub(crate) mod op_push_skew;
pub(crate) mod op_push_trans;
pub(crate) mod op_push_translate;
pub(crate) mod op_quad;

/// Stds
use std::rc::Rc;

/// 3rds
use egui_plot::PlotPoint;
use getset::{CopyGetters, Getters, MutGetters, Setters};

/// Crates
use crate::any_data::AnyData;
use crate::interfaces::{ICommandDescription, ICommandSyntax, IVisData};

/// Self
use super::syntax::CommonVecOpSyntax;
use super::VecLineData;
use op_cubi::CommonOpCUBI;
use op_end::CommonOpEND;
use op_line::CommonOpLINE;
use op_move::CommonOpMOVE;
use op_pop_trans::CommonOpPopTrans;
use op_push_rotate::CommonOpPushRotate;
use op_push_scale::CommonOpPushScale;
use op_push_skew::CommonOpPushSkew;
use op_push_trans::CommonOpPushTrans;
use op_push_translate::CommonOpPushTranslate;
use op_quad::CommonOpQUAD;

#[derive(Clone, Getters, CopyGetters, MutGetters, Setters)]
pub struct GenerateCtx {
    #[getset(get_copy = "pub", set = "pub", get_mut = "pub")]
    grouping: bool,

    #[getset(get_copy = "pub", set = "pub", get_mut = "pub")]
    cursor: PlotPoint,

    #[getset(set = "pub", get_mut = "pub")]
    local_trans: Vec<[[f64; 3]; 3]>,

    #[getset(get_copy = "pub", set = "pub", get_mut = "pub")]
    current_trans: [[f64; 3]; 3],
}

impl Default for GenerateCtx {
    fn default() -> Self {
        Self {
            grouping: false,
            cursor: PlotPoint::new(0.0, 0.0),
            local_trans: vec![],
            current_trans: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }
}

pub fn calc_trans_stack(trans_stack: &Vec<[[f64; 3]; 3]>) -> [[f64; 3]; 3] {
    fn mul_matrix(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
        let mut result = [[0.0; 3]; 3];

        for (i, iv) in a.iter().enumerate() {
            for j in 0..3 {
                for (k, kv) in b.iter().enumerate() {
                    result[i][j] += iv[k] * kv[j];
                }
            }
        }

        result
    }

    let mut res = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    for i in trans_stack {
        res = mul_matrix(&res, i);
    }
    res
}

pub fn process_point(argv: Rc<Vec<AnyData>>, matrix: [[f64; 3]; 3]) -> Vec<f64> {
    argv.chunks(2)
        .map(|x| {
            VecLineData::new(*x[0].cast_ref(), *x[1].cast_ref())
                .matrix(matrix)
                .cast::<VecLineData>()
        })
        .flat_map(|x| [x.x(), x.y()])
        .collect::<Vec<_>>()
}

impl ICommandSyntax for CommonVecOpSyntax {
    fn name(&self) -> &'static str {
        "CommonVecOpSyntax"
    }

    fn formats(&self) -> Vec<&'static dyn ICommandDescription> {
        vec![
            &CommonOpMOVE {},
            &CommonOpLINE {},
            &CommonOpQUAD {},
            &CommonOpCUBI {},
            &CommonOpEND {},
            &CommonOpPushTrans {},
            &CommonOpPopTrans {},
            &CommonOpPushScale {},
            &CommonOpPushRotate {},
            &CommonOpPushSkew {},
            &CommonOpPushTranslate {},
        ]
    }
}
