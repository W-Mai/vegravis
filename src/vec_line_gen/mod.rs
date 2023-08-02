use eframe::egui::plot::{PlotPoint, PlotPoints, Points};
use eframe::egui::Shape;

pub enum VecOpsType {
    VecOpMove,
    VecOpLine,
    VecOpQuad,
    VecOpCubi,
    VecOpEnd,
}

pub enum VecOps {
    VecOpMove(f64, f64),
    VecOpLine(f64, f64),
    VecOpQuad(f64, f64, f64, f64),
    VecOpCubi(f64, f64, f64, f64, f64, f64),
    VecOpEnd,
}

impl From<String> for VecOpsType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "move" => VecOpsType::VecOpMove,
            "line" => VecOpsType::VecOpLine,
            "quad" => VecOpsType::VecOpQuad,
            "cubi" => VecOpsType::VecOpCubi,
            "end" => VecOpsType::VecOpEnd,
            _ => panic!("Invalid VecOps"),
        }
    }
}

impl From<&str> for VecOpsType {
    fn from(value: &str) -> Self {
        value.to_owned().into()
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

    pub fn gen(&mut self) -> PlotPoints {
        let mut points = Vec::new();
        for op in &self.ops {
            match op {
                VecOps::VecOpMove(x, y) => {
                    self.cursor = PlotPoint::from([*x, *y]);
                }
                VecOps::VecOpLine(x, y) => {
                    points.push([self.cursor.x, self.cursor.y]);
                    points.push([*x, *y]);
                    self.cursor = PlotPoint::from([*x, *y]);
                }
                VecOps::VecOpEnd => {}
                _ => {}
            }
        }

        points.into_iter().collect()
    }
}
