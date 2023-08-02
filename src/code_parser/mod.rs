use crate::vec_line_gen::{VecOps, VecOpsType};

#[derive(Debug, Clone)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
    pub pos: usize,
}

#[derive(Debug, Clone)]
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

    fn read_number(&mut self) -> Result<f64, ParseError> {
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
        match number.parse() {
            Ok(n) => Ok(n),
            Err(_) => Err(ParseError { msg: "Invalid number".to_owned(), cursor: self.cursor.clone() }),
        }
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

    fn eat_comma(&mut self) -> Result<(), ParseError> {
        self.eat_whitespace();
        while self.curr_pos() < self.code.len() {
            let c = self.code.chars().nth(self.curr_pos()).unwrap();
            if c == ',' {
                self.cursor_next(c);
                break;
            } else {
                return Err(ParseError { msg: "Expected comma".to_owned(), cursor: self.cursor.clone() });
            }
        }
        self.eat_whitespace();
        Ok(())
    }

    pub fn parse(&mut self) -> Result<Vec<VecOps>, ParseError> {
        let mut ops = Vec::new();
        while self.curr_pos() < self.code.len() {
            let op = self.parse_op()?;
            ops.push(op);
        }
        Ok(ops)
    }

    fn parse_op(&mut self) -> Result<VecOps, ParseError> {
        let op_type: VecOpsType = self.read_ident().into();
        self.eat_comma()?;
        Ok(match op_type {
            VecOpsType::VecOpMove => self.parse_move()?,
            VecOpsType::VecOpLine => self.parse_line()?,
            VecOpsType::VecOpQuad => self.parse_quad()?,
            VecOpsType::VecOpCubi => self.parse_cubi()?,
            VecOpsType::VecOpEnd => self.parse_end()?,
        })
    }

    fn parse_move(&mut self) -> Result<VecOps, ParseError> {
        let x = self.read_number()?;
        self.eat_comma()?;
        let y = self.read_number()?;
        self.eat_comma()?;
        Ok(VecOps::VecOpMove(x, y))
    }

    fn parse_line(&mut self) -> Result<VecOps, ParseError> {
        let x = self.read_number()?;
        self.eat_comma()?;
        let y = self.read_number()?;
        self.eat_comma()?;
        Ok(VecOps::VecOpLine(x, y))
    }

    fn parse_quad(&mut self) -> Result<VecOps, ParseError> {
        let x1 = self.read_number()?;
        self.eat_comma()?;
        let y1 = self.read_number()?;
        self.eat_comma()?;
        let x2 = self.read_number()?;
        self.eat_comma()?;
        let y2 = self.read_number()?;
        self.eat_comma()?;
        Ok(VecOps::VecOpQuad(x1, y1, x2, y2))
    }

    fn parse_cubi(&mut self) -> Result<VecOps, ParseError> {
        let x1 = self.read_number()?;
        self.eat_comma()?;
        let y1 = self.read_number()?;
        self.eat_comma()?;
        let x2 = self.read_number()?;
        self.eat_comma()?;
        let y2 = self.read_number()?;
        self.eat_comma()?;
        let x3 = self.read_number()?;
        self.eat_comma()?;
        let y3 = self.read_number()?;
        self.eat_comma()?;
        Ok(VecOps::VecOpCubi(x1, y1, x2, y2, x3, y3))
    }

    fn parse_end(&mut self) -> Result<VecOps, ParseError> {
        Ok(VecOps::VecOpEnd)
    }
}
