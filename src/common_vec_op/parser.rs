use crate::common_vec_op::gen::VecLineGen;
use crate::interfaces::{Cursor, ICommandSyntax, IVisDataGenerator, ParseError};
use crate::syntax::CommonVecOpSyntax;

#[derive(Debug, Clone)]
pub struct CodeParser {
    pub code: String,
    pub cursor: Cursor,

    gen: VecLineGen,
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

impl TokenValue {
    fn into_string(self) -> Result<String, ParseError> {
        match self {
            TokenValue::Ident(s) => Ok(s),
            _ => Err(ParseError::default()),
        }
    }

    fn into_number(self) -> Result<f64, ParseError> {
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
    pub fn new(code: String, gen: VecLineGen) -> Self {
        Self { code, cursor: Cursor::default(), gen }
    }

    pub fn parse(&mut self) -> Result<&VecLineGen, ParseError> {
        self.eat_comments()?;
        while self.curr_pos() < self.code.len() {
            self.parse_op()?;
        }
        Ok(&self.gen)
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

    fn curr_ch(&self) -> char {
        self.code.chars().nth(self.curr_pos()).unwrap()
    }

    fn not_eof(&self) -> bool {
        self.curr_pos() < self.code.len()
    }

    fn read_ident(&mut self) -> ReadResult {
        let cur = self.curr_cur();
        let mut ident = String::new();
        while self.not_eof() {
            let c = self.curr_ch();
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
            let c = self.curr_ch();
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

    fn read_n_params(&mut self, n: usize) -> Result<Vec<f64>, ParseError> {
        let mut params = Vec::new();
        for _ in 0..n {
            self.eat_comments()?;
            let number = self.read_number()?.value.into_number()?;
            params.push(number);
            self.eat_comma()?;
        }
        Ok(params)
    }

    fn eat_whitespace(&mut self) {
        while self.not_eof() {
            let c = self.curr_ch();
            if c.is_whitespace() {
                self.cursor_next(c);
            } else {
                break;
            }
        }
    }

    fn eat_comma(&mut self) -> ReadResult {
        let cur = self.curr_cur();
        self.eat_comments()?;
        while self.not_eof() {
            let c = self.curr_ch();
            if c == ',' {
                self.cursor_next(c);
                break;
            } else {
                return Err(ParseError { msg: "Expected comma".to_owned(), cursor: cur });
            }
        }
        self.eat_comments()?;
        Ok(Token { value: TokenValue::Comma, cursor: (cur, self.curr_cur()) })
    }

    fn eat_comments(&mut self) -> ReadResult {
        self.eat_whitespace();
        let mut comment = self.eat_comment();
        loop {
            self.eat_whitespace();
            if self.check_comment().is_none() {
                break;
            } else {
                comment = self.eat_comment();
            }
        }
        comment
    }

    fn eat_comment(&mut self) -> ReadResult {
        let cur = self.curr_cur();
        match self.check_comment() {
            Some(CommentType::SingleLine) => {
                self.cursor_next('/');
                self.cursor_next('/');
                while self.not_eof() {
                    let c = self.curr_ch();
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
                        self.cursor_next(self.curr_ch());
                    }
                }
            }
            _ => {}
        }
        if !self.not_eof() && cur.pos != self.curr_pos() {
            return Err(ParseError { msg: "Invalid comment".to_owned(), cursor: cur });
        }
        let comment = self.code[cur.pos..self.curr_pos()].to_owned();
        Ok(Token { value: TokenValue::Comment(comment), cursor: (cur, self.curr_cur()) })
    }

    fn check_comment(&mut self) -> Option<CommentType> {
        let mut tmp_pos = self.curr_pos();
        let mut slash = false;
        let mut asterisk = false;

        while tmp_pos < self.code.len() {
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

    fn parse_op(&mut self) -> Result<(), ParseError> {
        self.eat_whitespace();
        let ident = self.read_ident()?;
        let ident_cur = ident.cursor.clone();
        let ident_string = ident.value.into_string()?;
        self.eat_comma()?;
        let cmd = CommonVecOpSyntax {}.match_command(ident_string.as_str());

        match cmd {
            Ok(dsc) => {
                let params = self.read_n_params(dsc.argc)?;
                return Ok(self.gen.add(dsc.pack(params)));
            }
            Err(maybe_cmd) => {
                if ident_string.len() == 0 {
                    Err(ParseError { msg: "Empty op type".to_owned(), cursor: ident_cur.0 })
                } else if maybe_cmd.len() > 0 {
                    Err(ParseError { msg: format!("Invalid op type '{}', maybe it is '{}'", ident_string, maybe_cmd), cursor: ident_cur.0 })
                } else {
                    Err(ParseError { msg: format!("Invalid op type '{}'", ident_string), cursor: ident_cur.0 })
                }
            }
        }
    }
}
