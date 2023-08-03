use std::str::FromStr;
use eframe::egui::plot::{PlotPoint};

pub enum VecOpsType {
    VecOpMove,
    VecOpLine,
    VecOpQuad,
    VecOpCubi,
    VecOpEnd,
}

#[derive(Clone)]
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
        match s {
            "move" => Ok(VecOpsType::VecOpMove),
            "line" => Ok(VecOpsType::VecOpLine),
            "quad" => Ok(VecOpsType::VecOpQuad),
            "cubi" => Ok(VecOpsType::VecOpCubi),
            "end" => Ok(VecOpsType::VecOpEnd),
            _ => Err(()),
        }
    }
}

pub struct VecLineGen {
    ops: Vec<VecOps>,
    cursor: PlotPoint,
}

impl VecLineGen {
    pub fn new(ops: Vec<VecOps>) -> Self {
        Self { ops, cursor: PlotPoint::from([0.0, 0.0]) }
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

    pub fn gen(&mut self) -> Vec<Vec<[f64; 2]>> {
        let mut points_total = Vec::new();
        let mut points = Vec::new();
        for op in &self.ops {
            match op {
                VecOps::VecOpMove(x, y) => {
                    self.cursor = PlotPoint::from([*x, *y]);
                    points_total.push(points);
                    points = Vec::new();
                    points.push([*x, *y]);
                }
                VecOps::VecOpLine(x, y) => {
                    points.push([self.cursor.x, self.cursor.y]);
                    points.push([*x, *y]);
                    self.cursor = PlotPoint::from([*x, *y]);
                }
                VecOps::VecOpQuad(x1, y1, x2, y2) => {
                    let mut t = 0.0;
                    while t < 1.0 {
                        let x = (1.0f64 - t).powi(2) * self.cursor.x + 2.0 * (1.0 - t) * t * x1 + t.powi(2) * x2;
                        let y = (1.0f64 - t).powi(2) * self.cursor.y + 2.0 * (1.0 - t) * t * y1 + t.powi(2) * y2;
                        points.push([x, y]);
                        t += 0.01;
                    }
                    self.cursor = PlotPoint::from([*x2, *y2]);
                }
                VecOps::VecOpCubi(x1, y1, x2, y2, x3, y3) => {
                    let mut t = 0.0;
                    while t < 1.0 {
                        let x = (1.0f64 - t).powi(3) * self.cursor.x + 3.0 * (1.0 - t).powi(2) * t * x1 + 3.0 * (1.0 - t) * t.powi(2) * x2 + t.powi(3) * x3;
                        let y = (1.0f64 - t).powi(3) * self.cursor.y + 3.0 * (1.0 - t).powi(2) * t * y1 + 3.0 * (1.0 - t) * t.powi(2) * y2 + t.powi(3) * y3;
                        points.push([x, y]);
                        t += 0.01;
                    }
                    self.cursor = PlotPoint::from([*x3, *y3]);
                }
                VecOps::VecOpEnd => {}
            }
        }
        if !points.is_empty() {
            points_total.push(points);
        }

        points_total
    }
}
