/// Stds
use std::fmt::Debug;
use std::ops::Range;

/// 3rds
use getset::{CopyGetters, Getters, MutGetters, Setters};

/// Crates
use crate::any_data::AnyData;
use crate::interfaces::{Command, ICommandSyntax, IVisData, IVisDataGenerator};

/// Self
use super::ops::GenerateCtx;
use super::syntax::CommonVecOpSyntax;

#[derive(Getters, Setters, MutGetters, CopyGetters, Default, Debug, Copy, Clone, PartialEq)]
pub struct VecLineData {
    #[getset(get_copy = "pub", set = "pub", get_mut = "pub")]
    x: f64,

    #[getset(get_copy = "pub", set = "pub", get_mut = "pub")]
    y: f64,
}

impl VecLineData {
    pub(crate) fn new(x: f64, y: f64) -> Self {
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

    fn generate(&self, range: Range<i64>) -> Vec<Vec<Box<dyn IVisData>>> {
        let mut gen_ctx = AnyData::new(GenerateCtx::default());
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

            if gen_ctx.cast_ref::<GenerateCtx>().grouping() {
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

        let world_matrix = gen_ctx.cast_ref::<GenerateCtx>().current_world_trans();
        points_total.iter_mut().for_each(|p| {
            p.iter_mut().for_each(|x| {
                *x = Box::new(x.matrix(world_matrix).cast::<VecLineData>());
            })
        });

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
