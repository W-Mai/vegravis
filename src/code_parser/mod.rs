use crate::vec_line_gen::{VecOps, VecOpsType};

pub struct CodeParser {
    pub code: String,
    pub cursor: usize,
}

impl CodeParser {
    pub fn new(code: String) -> Self {
        Self { code, cursor: 0 }
    }

    fn read_ident(&mut self) -> String {
        let mut ident = String::new();
        while self.cursor < self.code.len() {
            let c = self.code.chars().nth(self.cursor).unwrap();
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.cursor += 1;
            } else {
                break;
            }
        }
        ident
    }

    fn read_number(&mut self) -> f64 {
        let mut number = String::new();
        while self.cursor < self.code.len() {
            let c = self.code.chars().nth(self.cursor).unwrap();
            if c == '-' || c.is_numeric() || c == '.' {
                number.push(c);
                self.cursor += 1;
            } else {
                break;
            }
        }
        number.parse().unwrap()
    }

    fn eat_whitespace(&mut self) {
        while self.cursor < self.code.len() {
            let c = self.code.chars().nth(self.cursor).unwrap();
            if c.is_whitespace() {
                self.cursor += 1;
            } else {
                break;
            }
        }
    }

    fn eat_comma(&mut self) {
        self.eat_whitespace();
        while self.cursor < self.code.len() {
            let c = self.code.chars().nth(self.cursor).unwrap();
            if c == ',' {
                self.cursor += 1;
                break;
            } else {
                panic!("Expected comma");
            }
        }
        self.eat_whitespace();
    }

    pub fn parse(&mut self) -> Vec<VecOps> {
        let mut ops = Vec::new();
        while self.cursor < self.code.len() {
            let op = self.parse_op();
            ops.push(op);
        }
        ops
    }

    fn parse_op(&mut self) -> VecOps {
        let op_type: VecOpsType = self.read_ident().into();
        self.eat_comma();
        match op_type {
            VecOpsType::VecOpMove => self.parse_move(),
            VecOpsType::VecOpLine => self.parse_line(),
            VecOpsType::VecOpQuad => self.parse_quad(),
            VecOpsType::VecOpCubi => self.parse_cubi(),
            VecOpsType::VecOpEnd => self.parse_end(),
        }
    }

    fn parse_move(&mut self) -> VecOps {
        let x = self.read_number();
        self.eat_comma();
        let y = self.read_number();
        self.eat_comma();
        VecOps::VecOpMove(x, y)
    }

    fn parse_line(&mut self) -> VecOps {
        let x = self.read_number();
        self.eat_comma();
        let y = self.read_number();
        self.eat_comma();
        VecOps::VecOpLine(x, y)
    }

    fn parse_quad(&mut self) -> VecOps {
        let x1 = self.read_number();
        self.eat_comma();
        let y1 = self.read_number();
        self.eat_comma();
        let x2 = self.read_number();
        self.eat_comma();
        let y2 = self.read_number();
        self.eat_comma();
        VecOps::VecOpQuad(x1, y1, x2, y2)
    }

    fn parse_cubi(&mut self) -> VecOps {
        let x1 = self.read_number();
        self.eat_comma();
        let y1 = self.read_number();
        self.eat_comma();
        let x2 = self.read_number();
        self.eat_comma();
        let y2 = self.read_number();
        self.eat_comma();
        let x3 = self.read_number();
        self.eat_comma();
        let y3 = self.read_number();
        self.eat_comma();
        VecOps::VecOpCubi(x1, y1, x2, y2, x3, y3)
    }

    fn parse_end(&mut self) -> VecOps {
        self.read_ident();
        VecOps::VecOpEnd
    }
}
