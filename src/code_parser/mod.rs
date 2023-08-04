use std::process::id;
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

#[derive(Debug, Clone)]
pub struct CodeParser {
    pub code: String,
    pub cursor: Cursor,
}

#[derive(Debug, Clone)]
enum CommentType {
    SingleLine,
    MultiLineStart,
    MultiLineEnd,
}

#[derive(Debug, Clone)]
enum TokenValue {
    Ident(String),
    Number(f64),
    Comment(String),
    Comma,
}

impl Default for ParseError {
    fn default() -> Self {
        Self { msg: "Internal Error".to_owned(), cursor: Cursor { row: 0, col: 0, pos: 0 } }
    }
}

impl TokenValue {
    fn as_string(self) -> Result<String, ParseError> {
        match self {
            TokenValue::Ident(s) => Ok(s),
            _ => Err(ParseError::default()),
        }
    }

    fn as_number(self) -> Result<f64, ParseError> {
        match self {
            TokenValue::Number(f) => Ok(f),
            _ => Err(ParseError::default()),
        }
    }
}

// left close right open range [l, r)
type Span = (Cursor, Cursor);

#[derive(Debug, Clone)]
struct Token {
    value: TokenValue,
    cursor: Span,
}

type ReadResult = Result<Token, ParseError>;

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

    fn curr_cur(&self) -> Cursor {
        self.cursor.clone()
    }

    fn curr_pos(&self) -> usize {
        self.cursor.pos
    }

    fn not_eof(&self) -> bool {
        self.curr_pos() < self.code.len()
    }

    fn read_ident(&mut self) -> ReadResult {
        let cur = self.curr_cur();
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
        Ok(Token { value: TokenValue::Ident(ident), cursor: (cur, self.curr_cur()) })
    }

    fn read_number(&mut self) -> ReadResult {
        let cur = self.curr_cur();
        let mut number = String::new();
        while self.not_eof() {
            let c = self.code.chars().nth(self.curr_pos()).unwrap();
            if c == '-' || c.is_numeric() || c == '.' {
                number.push(c);
                self.cursor_next(c);
            } else {
                break;
            }
        }
        match number.parse() {
            Ok(n) => Ok(Token { value: TokenValue::Number(n), cursor: (cur, self.curr_cur()) }),
            Err(_) => {
                Err(ParseError { msg: format!("Invalid number '{}'", number), cursor: cur })
            }
        }
    }

    fn read_n_params(&mut self, n: usize) -> Result<Vec<Token>, ParseError> {
        let mut params = Vec::new();
        for _ in 0..n {
            self.eat_comment();
            let number = self.read_number()?;
            params.push(number);
            self.eat_comma()?;
        }
        Ok(params)
    }

    fn eat_whitespace(&mut self) {
        while self.not_eof() {
            let c = self.code.chars().nth(self.curr_pos()).unwrap();
            if c.is_whitespace() {
                self.cursor_next(c);
            } else {
                break;
            }
        }
    }

    fn eat_comma(&mut self) -> ReadResult {
        let cur = self.curr_cur();
        self.eat_comment();
        while self.not_eof() {
            let c = self.code.chars().nth(self.curr_pos()).unwrap();
            if c == ',' {
                self.cursor_next(c);
                break;
            } else {
                return Err(ParseError { msg: "Expected comma".to_owned(), cursor: cur });
            }
        }
        self.eat_comment();
        Ok(Token { value: TokenValue::Comma, cursor: (cur, self.curr_cur()) })
    }

    fn eat_comment(&mut self) {
        self.eat_whitespace();
        match self.check_comment() {
            Some(CommentType::SingleLine) => {
                self.cursor_next('/');
                self.cursor_next('/');
                while self.not_eof() {
                    let c = self.code.chars().nth(self.curr_pos()).unwrap();
                    if c != '\n' {
                        self.cursor_next(c);
                    } else {
                        self.cursor_next(c);
                        break;
                    }
                }
            }
            Some(CommentType::MultiLineStart) => {
                self.cursor_next('/');
                self.cursor_next('*');

                while self.not_eof() {
                    if let Some(CommentType::MultiLineEnd) = self.check_comment() {
                        self.cursor_next('*');
                        self.cursor_next('/');
                        break;
                    } else {
                        self.cursor_next(self.code.chars().nth(self.curr_pos()).unwrap());
                    }
                }
            }
            _ => {}
        }
        self.eat_whitespace();
    }

    fn check_comment(&mut self) -> Option<CommentType> {
        let mut tmp_pos = self.curr_pos();
        let mut slash = false;
        let mut asterisk = false;

        while self.not_eof() {
            let c = self.code.chars().nth(tmp_pos).unwrap();
            if c == '/' {
                if slash {
                    return Some(CommentType::SingleLine);
                }
                if asterisk {
                    return Some(CommentType::MultiLineEnd);
                }
                slash = true;
            } else if c == '*' {
                if slash {
                    return Some(CommentType::MultiLineStart);
                }
                if asterisk {
                    return None;
                }
                asterisk = true;
            } else {
                return None;
            }
            tmp_pos += 1;
        }
        None
    }

    pub fn parse(&mut self) -> Result<Vec<VecOps>, ParseError> {
        let mut ops = Vec::new();
        self.eat_comment();
        while self.curr_pos() < self.code.len() {
            let op = self.parse_op()?;
            ops.push(op);
        }
        Ok(ops)
    }

    fn parse_op(&mut self) -> Result<VecOps, ParseError> {
        let ident = self.read_ident()?;
        let ident_cur = ident.cursor.clone();
        let ident_string = ident.value.as_string()?;
        self.eat_comma()?;
        match ident_string.parse() {
            Ok(VecOpsType::VecOpMove) => self.parse_move(),
            Ok(VecOpsType::VecOpLine) => self.parse_line(),
            Ok(VecOpsType::VecOpQuad) => self.parse_quad(),
            Ok(VecOpsType::VecOpCubi) => self.parse_cubi(),
            Ok(VecOpsType::VecOpEnd) => self.parse_end(),
            _ => {
                Err(ParseError { msg: format!("Invalid op type '{}'", ident_string), cursor: ident_cur.0 })
            }
        }
    }

    fn parse_move(&mut self) -> Result<VecOps, ParseError> {
        let x = self.read_number()?.value.as_number()?;
        self.eat_comma()?;
        let y = self.read_number()?.value.as_number()?;
        self.eat_comma()?;
        Ok(VecOps::VecOpMove(x, y))
    }

    fn parse_line(&mut self) -> Result<VecOps, ParseError> {
        let x = self.read_number()?.value.as_number()?;
        self.eat_comma()?;
        let y = self.read_number()?.value.as_number()?;
        self.eat_comma()?;
        Ok(VecOps::VecOpLine(x, y))
    }

    fn parse_quad(&mut self) -> Result<VecOps, ParseError> {
        let x1 = self.read_number()?.value.as_number()?;
        self.eat_comma()?;
        let y1 = self.read_number()?.value.as_number()?;
        self.eat_comma()?;
        let x2 = self.read_number()?.value.as_number()?;
        self.eat_comma()?;
        let y2 = self.read_number()?.value.as_number()?;
        self.eat_comma()?;
        Ok(VecOps::VecOpQuad(x1, y1, x2, y2))
    }

    fn parse_cubi(&mut self) -> Result<VecOps, ParseError> {
        let x1 = self.read_number()?.value.as_number()?;
        self.eat_comma()?;
        let y1 = self.read_number()?.value.as_number()?;
        self.eat_comma()?;
        let x2 = self.read_number()?.value.as_number()?;
        self.eat_comma()?;
        let y2 = self.read_number()?.value.as_number()?;
        self.eat_comma()?;
        let x3 = self.read_number()?.value.as_number()?;
        self.eat_comma()?;
        let y3 = self.read_number()?.value.as_number()?;
        self.eat_comma()?;
        Ok(VecOps::VecOpCubi(x1, y1, x2, y2, x3, y3))
    }

    fn parse_end(&mut self) -> Result<VecOps, ParseError> {
        self.eat_comma()?;
        Ok(VecOps::VecOpEnd)
    }
}
