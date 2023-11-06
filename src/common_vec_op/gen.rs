use std::ops::{Range};
use egui_plot::PlotPoint;
use crate::any_data::AnyData;
use crate::interfaces::{Command, IVisData, IVisDataGenerator};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VecLineData {
    x: f64,
    y: f64,
}

impl VecLineData {
    fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
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
            let [
            a, b, c,
            d, e, f,
            g, h, i
            ] = [
                matrix[0][0], matrix[0][1], matrix[0][2],
                matrix[1][0], matrix[1][1], matrix[1][2],
                matrix[2][0], matrix[2][1], matrix[2][2],
            ];
            let [x, y, z] = point;
            [
                a * x + b * y + c * z,
                d * x + e * y + f * z,
                g * x + h * y + i * z,
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

    fn gen(&self, range: Range<i64>) -> Vec<Vec<Box<dyn IVisData>>> {
        let mut cursor = PlotPoint::new(0.0, 0.0);
        let mut points_total = Vec::new();
        let mut points: Vec<Box<dyn IVisData>> = Vec::new();
        let mut counter = 0i64;

        for op in &self.ops {
            if !range.contains(&counter) {
                continue;
            }
            match op.dsc.name {
                "MOVE" => {
                    cursor = PlotPoint::from([*op.argv[0].cast_ref(), *op.argv[1].cast_ref()]);
                    if points_total.len() == 0 {
                        points.push(Box::new(VecLineData::new(0.0, 0.0)));
                    }
                    points_total.push(points);
                    points = Vec::new();
                    points.push(Box::new(VecLineData::new(*op.argv[0].cast_ref(), *op.argv[1].cast_ref())));
                }
                "LINE" => {
                    points.push(Box::new(VecLineData::new(*op.argv[0].cast_ref(), *op.argv[1].cast_ref())));
                    points.push(Box::new(VecLineData::new(*op.argv[0].cast_ref(), *op.argv[1].cast_ref())));
                    cursor = PlotPoint::from([*op.argv[0].cast_ref(), *op.argv[1].cast_ref()]);
                }
                "QUAD" => {
                    let [x1, y1, x2, y2] = [*op.argv[0].cast_ref(), *op.argv[1].cast_ref(), *op.argv[2].cast_ref(), *op.argv[3].cast_ref()];
                    let mut t = 0.0;
                    while t < 1.0 {
                        let x = (1.0f64 - t).powi(2) * cursor.x + 2.0 * (1.0 - t) * t * x1 + t.powi(2) * x2;
                        let y = (1.0f64 - t).powi(2) * cursor.y + 2.0 * (1.0 - t) * t * y1 + t.powi(2) * y2;
                        points.push(Box::new(VecLineData::new(x, y)));
                        t += 0.01;
                    }
                    cursor = PlotPoint::from([x2, y2]);
                }
                "CUBI" => {
                    let [x1, y1, x2, y2, x3, y3] = [*op.argv[0].cast_ref(), *op.argv[1].cast_ref(), *op.argv[2].cast_ref(), *op.argv[3].cast_ref(), *op.argv[4].cast_ref(), *op.argv[5].cast_ref()];
                    let mut t = 0.0;
                    while t < 1.0 {
                        let x = (1.0f64 - t).powi(3) * cursor.x + 3.0 * (1.0 - t).powi(2) * t * x1 + 3.0 * (1.0 - t) * t.powi(2) * x2 + t.powi(3) * x3;
                        let y = (1.0f64 - t).powi(3) * cursor.y + 3.0 * (1.0 - t).powi(2) * t * y1 + 3.0 * (1.0 - t) * t.powi(2) * y2 + t.powi(3) * y3;
                        points.push(Box::new(VecLineData::new(x, y)));
                        t += 0.01;
                    }
                    cursor = PlotPoint::from([x3, y3]);
                }
                "END" => {}
                _ => {
                    unreachable!("Invalid command: {:?}", op);
                }
            }

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
}

impl Default for VecLineGen {
    fn default() -> Self {
        VecLineGen::new(vec![])
    }
}
