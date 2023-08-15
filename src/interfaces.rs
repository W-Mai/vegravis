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

pub trait IDataSource<ST> {
    fn new(code: ST) -> Self;
    fn get(&self, name: &str) -> Option<ST>;
}

pub trait IParser<PT: Numeric, ST, CT: Numeric, VDT: IVisData<PT>, DST: IDataSource<ST>, G: IVisDataGenerator<CT, PT, VDT>> {
    fn new(code: DST, gen: G) -> Self;

    fn parse(&mut self) -> Result<&G, ParseError>;
}

pub trait IEncoder {
    fn encode(&self, input: &str) -> String;
}

pub trait IVisDataGenerator<CT: Numeric, PT: Numeric, VDT: IVisData<PT>> {
    fn add(&mut self, op: Command<CT>);

    fn gen(&self, range: Range<i64>) -> Vec<Vec<VDT>>;

    fn len(&self) -> usize;
}

pub trait IVisData<PT: Numeric> {
    fn new(x: PT, y: PT) -> Self;

    fn pos(&self) -> [PT; 2];

    fn matrix(&self, matrix: [[PT; 3]; 3]) -> Self;
}

pub trait IVisualizer<PT: Numeric, VDT: IVisData<PT>> {
    fn plot(&self, input: VDT);
}

pub trait ICodeEditor<ST, DST: IDataSource<ST>, CST: ICommandSyntax> {
    fn show(&self, ui: &mut egui::Ui, code: &mut DST, format: CST) -> egui::Response;
}
