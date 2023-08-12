use std::ops::{Range};
use std::str::FromStr;
use eframe::egui::plot::{PlotPoint};
use levenshtein::levenshtein;
use log::error;
use crate::interfaces::{CommandDescription, ICommandSyntax};
use crate::syntax::CommonVecOpSyntax;

pub enum VecOpsType {
    VecOpMove,
    VecOpLine,
    VecOpQuad,
    VecOpCubi,
    VecOpEnd,

    VecOpInvalid(String),
}

#[derive(Debug, Clone)]
pub enum VecOps {
    VecOpMove(f64, f64),
    VecOpLine(f64, f64),
    VecOpQuad(f64, f64, f64, f64),
    VecOpCubi(f64, f64, f64, f64, f64, f64),
    VecOpEnd,
}

impl FromStr for VecOpsType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m = CommonVecOpSyntax {}.match_command(s);

        match m {
            Ok(&CommandDescription { name, argc: _argc }) => {
                match name {
                    "MOVE" => Ok(VecOpsType::VecOpMove),
                    "LINE" => Ok(VecOpsType::VecOpLine),
                    "QUAD" => Ok(VecOpsType::VecOpQuad),
                    "CUBI" => Ok(VecOpsType::VecOpCubi),
                    "END" => Ok(VecOpsType::VecOpEnd),
                    _ => {
                        unreachable!()
                    }
                }
            }
            Err(maybe_cmd) => {
                if maybe_cmd.len() > 0 {
                    Ok(VecOpsType::VecOpInvalid(maybe_cmd.to_owned()))
                } else {
                    Err(())
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct VecLineGen {
    ops: Vec<VecOps>,
}

impl VecLineGen {
    pub fn new(ops: Vec<VecOps>) -> Self {
        Self { ops }
    }

    pub fn add(&mut self, op: VecOps) {
        self.ops.push(op);
    }

    pub fn add_move(&mut self, x: f64, y: f64) {
        self.add(VecOps::VecOpMove(x, y));
    }

    pub fn add_line(&mut self, x: f64, y: f64) {
        self.add(VecOps::VecOpLine(x, y));
    }

    pub fn add_quad(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.add(VecOps::VecOpQuad(x1, y1, x2, y2));
    }

    pub fn add_cubi(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        self.add(VecOps::VecOpCubi(x1, y1, x2, y2, x3, y3));
    }

    pub fn add_end(&mut self) {
        self.add(VecOps::VecOpEnd);
    }

    pub fn gen(&self, range: Range<i64>) -> Vec<Vec<[f64; 2]>> {
        let mut cursor = PlotPoint::new(0.0, 0.0);
        let mut points_total = Vec::new();
        let mut points = Vec::new();
        let mut counter = 0i64;

        for op in &self.ops {
            if !range.contains(&counter) {
                continue;
            }
            match op {
                VecOps::VecOpMove(x, y) => {
                    cursor = PlotPoint::from([*x, *y]);
                    if points_total.len() == 0 {
                        points.push([0.0, 0.0]);
                    }
                    points_total.push(points);
                    points = Vec::new();
                    points.push([*x, *y]);
                }
                VecOps::VecOpLine(x, y) => {
                    points.push([cursor.x, cursor.y]);
                    points.push([*x, *y]);
                    cursor = PlotPoint::from([*x, *y]);
                }
                VecOps::VecOpQuad(x1, y1, x2, y2) => {
                    let mut t = 0.0;
                    while t < 1.0 {
                        let x = (1.0f64 - t).powi(2) * cursor.x + 2.0 * (1.0 - t) * t * x1 + t.powi(2) * x2;
                        let y = (1.0f64 - t).powi(2) * cursor.y + 2.0 * (1.0 - t) * t * y1 + t.powi(2) * y2;
                        points.push([x, y]);
                        t += 0.01;
                    }
                    cursor = PlotPoint::from([*x2, *y2]);
                }
                VecOps::VecOpCubi(x1, y1, x2, y2, x3, y3) => {
                    let mut t = 0.0;
                    while t < 1.0 {
                        let x = (1.0f64 - t).powi(3) * cursor.x + 3.0 * (1.0 - t).powi(2) * t * x1 + 3.0 * (1.0 - t) * t.powi(2) * x2 + t.powi(3) * x3;
                        let y = (1.0f64 - t).powi(3) * cursor.y + 3.0 * (1.0 - t).powi(2) * t * y1 + 3.0 * (1.0 - t) * t.powi(2) * y2 + t.powi(3) * y3;
                        points.push([x, y]);
                        t += 0.01;
                    }
                    cursor = PlotPoint::from([*x3, *y3]);
                }
                VecOps::VecOpEnd => {}
            }

            counter += 1;
        }
        if !points.is_empty() {
            points_total.push(points);
        }

        points_total
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }
}

impl Default for VecLineGen {
    fn default() -> Self {
        VecLineGen::new(vec![])
    }
}
