use std::ops::{Range};
use eframe::egui::plot::{PlotPoint};
use crate::interfaces::{Command, IVisData, IVisDataGenerator};

#[derive(Debug, Clone)]
pub enum VecOps {
    VecOpMove(f64, f64),
    VecOpLine(f64, f64),
    VecOpQuad(f64, f64, f64, f64),
    VecOpCubi(f64, f64, f64, f64, f64, f64),
    VecOpEnd,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VecLineData {
    x: f64,
    y: f64,
}

impl IVisData<f64> for VecLineData {
    fn new(x: f64, y: f64) -> Self {
        VecLineData {
            x,
            y,
        }
    }

    fn pos(&self) -> [f64; 2] {
        [self.x, self.y]
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

    fn add(&mut self, op: VecOps) {
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

    fn gen(&self, range: Range<i64>) -> Vec<Vec<[f64; 2]>> {
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
}

impl IVisDataGenerator<f64, f64, VecLineData> for VecLineGen {
    fn add(&mut self, op: Command<f64>) {
        let dsc = op.dsc;
        let argv = op.argv;
        let cmd_name = dsc.name;

        match cmd_name {
            "MOVE" => {
                self.add_move(argv[0], argv[1]);
            }
            "LINE" => {
                self.add_line(argv[0], argv[1]);
            }
            "QUAD" => {
                self.add_quad(argv[0], argv[1], argv[2], argv[3])
            }
            "CUBI" => {
                self.add_cubi(argv[0], argv[1], argv[2], argv[3], argv[4], argv[5]);
            }
            "END" => {
                self.add_end();
            }
            _ => {}
        }
    }

    fn gen(&self, range: Range<i64>) -> Vec<Vec<VecLineData>> {
        let data = self.gen(range);
        let mut ret = Vec::new();

        for line in data {
            let mut vec_line_datas = Vec::new();
            for pos in line {
                vec_line_datas.push(VecLineData::new(pos[0], pos[1]));
            }
            ret.push(vec_line_datas);
        }

        ret
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
