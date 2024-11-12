use crate::any_data::AnyData;
use crate::common_vec_op::syntax::CommonVecOpSyntax;
use crate::interfaces::{
    Command, ICommandDescription, ICommandSyntax, IVisData, IVisDataGenerator,
};
use egui_plot::PlotPoint;
use std::fmt::Debug;
use std::ops::Range;
use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VecLineData {
    x: f64,
    y: f64,
}

impl VecLineData {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl IVisData for VecLineData {
    fn pos(&self) -> [AnyData; 2] {
        [AnyData::new(self.x), AnyData::new(self.y)]
    }

    fn is_same(&self, another_data: &dyn IVisData) -> bool {
        let another_pos = another_data.pos();
        self.x == *another_pos[0].cast_ref() && self.y == *another_pos[1].cast_ref()
    }

    fn matrix(&self, matrix: [[f64; 3]; 3]) -> AnyData {
        fn mul_point(matrix: [[f64; 3]; 3], point: [f64; 3]) -> [f64; 3] {
            let [a, b, c, d, e, f, g, h, i] = [
                matrix[0][0],
                matrix[0][1],
                matrix[0][2],
                matrix[1][0],
                matrix[1][1],
                matrix[1][2],
                matrix[2][0],
                matrix[2][1],
                matrix[2][2],
            ];
            let [x, y, z] = point;
            let z_ = g * x + h * y + i * z;
            [
                (a * x + b * y + c * z) / z_,
                (d * x + e * y + f * z) / z_,
                1.0,
            ]
        }
        let [x, y, _] = mul_point(matrix, [self.x, self.y, 1.0]);
        AnyData::new(VecLineData::new(x, y))
    }
}

struct GenerateCtx {
    grouping: bool,
    cursor: PlotPoint,
    local_trans: Vec<[[f64; 3]; 3]>,
}

#[derive(Debug, Clone)]
pub struct VecLineGen {
    ops: Vec<Command>,
}

impl VecLineGen {
    pub fn new(ops: Vec<Command>) -> Self {
        Self { ops }
    }
}

impl IVisDataGenerator for VecLineGen {
    fn add(&mut self, op: Command) {
        self.ops.push(op);
    }

    fn gen(&self, range: Range<i64>) -> Vec<Vec<Box<dyn IVisData>>> {
        let mut gen_ctx = AnyData::new(GenerateCtx {
            grouping: false,
            cursor: PlotPoint::new(0.0, 0.0),
            local_trans: vec![],
        });
        let mut points_total = vec![];
        let mut points: Vec<Box<dyn IVisData>> = vec![];
        let mut counter = 0i64;

        let p: Box<dyn IVisData> = Box::new(VecLineData::new(0.0, 0.0));
        points_total.push(vec![p]);

        for op in &self.ops {
            if !range.contains(&counter) {
                continue;
            }

            let converted = AnyData::convert_from_vec::<VecLineData>(op.operate(&mut gen_ctx));

            if gen_ctx.cast_ref::<GenerateCtx>().grouping {
                points.append(
                    &mut converted
                        .iter()
                        .map(|v| {
                            let res: Box<dyn IVisData> = Box::new(*v);
                            res
                        })
                        .collect(),
                );
                counter += 1;
                continue;
            }
            points_total.push(points);
            points = vec![];

            counter += 1;
        }
        if !points.is_empty() {
            points_total.push(points);
        }

        points_total
    }

    fn len(&self) -> usize {
        self.ops.len()
    }

    fn command_syntax(&self) -> &'static dyn ICommandSyntax {
        &CommonVecOpSyntax {}
    }
}

impl Default for VecLineGen {
    fn default() -> Self {
        VecLineGen::new(vec![])
    }
}

// ==============

struct CommonOpMOVE;

struct CommonOpLINE;

struct CommonOpQUAD;

struct CommonOpCUBI;

struct CommonOpEND;

struct CommonOpPushTrans;

struct CommonOpPopTrans;

impl ICommandDescription for CommonOpMOVE {
    fn name(&self) -> Vec<&str> {
        ["MOVE"].into()
    }

    fn argc(&self) -> usize {
        2
    }

    fn operate(&self, ctx: &mut AnyData, argv: Rc<Vec<AnyData>>) -> Vec<AnyData> {
        let ctx = ctx.cast_mut::<GenerateCtx>();
        let nums = [*argv[0].cast_ref(), *argv[1].cast_ref()];

        let points = vec![VecLineData::new(nums[0], nums[1])];

        ctx.grouping = false;
        ctx.cursor = PlotPoint::from(nums);

        AnyData::convert_to_vec(points)
    }
}

impl ICommandDescription for CommonOpLINE {
    fn name(&self) -> Vec<&str> {
        ["LINE"].into()
    }

    fn argc(&self) -> usize {
        2
    }

    fn operate(&self, ctx: &mut AnyData, argv: Rc<Vec<AnyData>>) -> Vec<AnyData> {
        let ctx = ctx.cast_mut::<GenerateCtx>();
        let nums = [*argv[0].cast_ref(), *argv[1].cast_ref()];

        let points = vec![
            VecLineData::new(ctx.cursor.x, ctx.cursor.y),
            VecLineData::new(nums[0], nums[1]),
        ];

        ctx.grouping = true;
        ctx.cursor = PlotPoint::from(nums);

        AnyData::convert_to_vec(points)
    }
}

impl ICommandDescription for CommonOpQUAD {
    fn name(&self) -> Vec<&str> {
        ["QUAD"].into()
    }

    fn argc(&self) -> usize {
        4
    }

    fn operate(&self, ctx: &mut AnyData, argv: Rc<Vec<AnyData>>) -> Vec<AnyData> {
        let ctx = ctx.cast_mut::<GenerateCtx>();
        let [x1, y1, x2, y2] = [
            *argv[0].cast_ref(),
            *argv[1].cast_ref(),
            *argv[2].cast_ref(),
            *argv[3].cast_ref(),
        ];

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

impl ICommandDescription for CommonOpCUBI {
    fn name(&self) -> Vec<&str> {
        ["CUBI", "CUBIC"].into()
    }

    fn argc(&self) -> usize {
        6
    }

    fn operate(&self, ctx: &mut AnyData, argv: Rc<Vec<AnyData>>) -> Vec<AnyData> {
        let ctx = ctx.cast_mut::<GenerateCtx>();
        let [x1, y1, x2, y2, x3, y3] = [
            *argv[0].cast_ref(),
            *argv[1].cast_ref(),
            *argv[2].cast_ref(),
            *argv[3].cast_ref(),
            *argv[4].cast_ref(),
            *argv[5].cast_ref(),
        ];

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

        ctx.local_trans.push(trans_matrix);

        vec![]
    }
}

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
        vec![]
    }
}

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
        ]
    }
}
