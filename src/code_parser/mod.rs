use crate::vec_line_gen::{VecOps, VecOpsType};

#[derive(Debug)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
    pub pos: usize,
}

#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
    pub cursor: Cursor,
}

pub struct CodeParser {
    pub code: String,
    pub cursor: Cursor,
}

impl CodeParser {
    pub fn new(code: String) -> Self {
        Self { code, cursor: Cursor { row: 0, col: 0, pos: 0 } }
    }

    fn cursor_next(&mut self, c: char) {
        self.cursor.pos += 1;
        self.cursor.col += 1;
        if c == '\n' {
            self.cursor.row += 1;
            self.cursor.col = 0;
        }
    }

    fn curr_pos(&self) -> usize {
        self.cursor.pos
    }

    fn not_eof(&self) -> bool {
        self.curr_pos() < self.code.len()
    }

    fn read_ident(&mut self) -> String {
        let mut ident = String::new();
        while self.not_eof() {
            let c = self.code.chars().nth(self.curr_pos()).unwrap();
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.cursor_next(c);
            } else {
                break;
            }
        }
        ident
    }

    fn read_number(&mut self) -> f64 {
        let mut number = String::new();
        while self.curr_pos() < self.code.len() {
            let c = self.code.chars().nth(self.curr_pos()).unwrap();
            if c == '-' || c.is_numeric() || c == '.' {
                number.push(c);
                self.cursor_next(c);
            } else {
                break;
            }
        }
        number.parse().unwrap()
    }

    fn eat_whitespace(&mut self) {
        while self.curr_pos() < self.code.len() {
            let c = self.code.chars().nth(self.curr_pos()).unwrap();
            if c.is_whitespace() {
                self.cursor_next(c);
            } else {
                break;
            }
        }
    }

    fn eat_comma(&mut self) {
        self.eat_whitespace();
        while self.curr_pos() < self.code.len() {
            let c = self.code.chars().nth(self.curr_pos()).unwrap();
            if c == ',' {
                self.cursor_next(c);
                break;
            } else {
                panic!("Expected comma");
            }
        }
        self.eat_whitespace();
    }

    pub fn parse(&mut self) -> Vec<VecOps> {
        let mut ops = Vec::new();
        while self.curr_pos() < self.code.len() {
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
