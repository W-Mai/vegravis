use std::cell::RefMut;
use std::collections::HashSet;
use std::ops::Range;
use eframe::egui;
use eframe::emath::Numeric;
use egui_code_editor::Syntax;
use levenshtein::levenshtein;

#[derive(Debug, Clone)]
pub struct CommandDescription {
    pub name: &'static str,
    pub argc: usize,
}

impl CommandDescription {
    pub fn pack<T: Numeric>(&'static self, argv: Vec<T>) -> Command<T> {
        assert_eq!(argv.len(), self.argc);

        Command {
            dsc: self,
            argv,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Command<T> {
    pub dsc: &'static CommandDescription,
    pub argv: Vec<T>,
}

#[derive(Default, Debug, Clone)]
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

impl Default for ParseError {
    fn default() -> Self {
        Self { msg: "Internal Error".to_owned(), cursor: Cursor::default() }
    }
}

pub trait ICommandSyntax {
    fn name(&self) -> &'static str;

    fn case_sensitive(&self) -> bool {
        false
    }
    fn formats(&self) -> Vec<&'static CommandDescription>;
    fn syntax(&self) -> Syntax {
        let keywords = self.formats().iter().map(|cmd| cmd.name).collect::<HashSet<&str>>();
        let types = HashSet::new();
        let special = HashSet::new();

        Syntax {
            language: self.name(),
            case_sensitive: self.case_sensitive(),
            comment: "//",
            comment_multiline: ["/*", "*/"],
            keywords,
            types,
            special,
        }
    }

    fn match_command(&self, cmd: &str) -> Result<&'static CommandDescription, &str> {
        let cmd = cmd.to_owned();
        let cmd = if self.case_sensitive() { cmd } else { cmd.to_uppercase() };
        let cmd = cmd.as_str();
        for desc in self.formats() {
            if desc.name == cmd {
                return Ok(desc);
            }
        }
        let mut dists = vec![];
        for desc in self.formats() {
            dists.push((levenshtein(cmd, desc.name), desc.name));
        }
        dists.sort_by(|a, b| a.0.cmp(&b.0));
        let mut dists = dists.into_iter();
        let (dist, op) = dists.next().unwrap();
        if dist >= 3 {
            return Err("");
        }
        Err(op)
    }
}

pub trait IDataSource: PartialEq {
    type ST;

    fn new(code: Self::ST) -> Self;
    fn get(&self, name: &str) -> Option<Self::ST>;

    fn get_ref(&self, name: &str) -> Option<RefMut<Self::ST>>;
}

pub trait IParser {
    type DST: IDataSource;
    type G: IVisDataGenerator;

    fn new(code: Self::DST, gen: Self::G) -> Self;

    fn parse(&mut self) -> Result<&Self::G, ParseError>;
}

pub trait IEncoder {
    fn encode(&self, input: &str) -> String;
}

pub trait IVisDataGenerator {
    type CT;
    type VDT: IVisData;

    fn add(&mut self, op: Command<Self::CT>);

    fn gen(&self, range: Range<i64>) -> Vec<Vec<Self::VDT>>;

    fn len(&self) -> usize;
}

pub trait IVisData {
    type PT: Numeric;

    fn new(x: Self::PT, y: Self::PT) -> Self;

    fn pos(&self) -> [Self::PT; 2];

    fn matrix(&self, matrix: [[Self::PT; 3]; 3]) -> Self;
}

pub trait IVisualizer {
    type VDT: IVisData;

    fn new(transform: [[f64; 3]; 3]) -> Self;
    fn plot(&self, ui: &mut egui::Ui, input: Vec<Vec<Self::VDT>>, has_error: bool, show_inter_dash: bool, colorful_block: bool);

    fn transform(&mut self, matrix: [[f64; 3]; 3]);
}

pub trait ICodeEditor {
    type DST: IDataSource;
    type CST: ICommandSyntax;

    fn show(&self, ui: &mut egui::Ui, code: &mut Self::DST, format: Self::CST) -> egui::Response;
}
