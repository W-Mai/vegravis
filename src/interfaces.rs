use std::cell::RefMut;
use std::collections::HashSet;
use std::ops::Range;
use std::rc::Rc;
use dyn_clone::DynClone;
use eframe::egui;
use egui_code_editor::Syntax;
use levenshtein::levenshtein;
use crate::any_data::AnyData;

#[derive(Debug, Clone)]
pub struct CommandDescription {
    pub name: &'static str,
    pub argc: usize,
}

impl CommandDescription {
    pub fn pack(&'static self, argv: Vec<AnyData>) -> Command {
        assert_eq!(argv.len(), self.argc);

        Command {
            dsc: self,
            argv: Rc::new(argv),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub dsc: &'static CommandDescription,
    pub argv: Rc<Vec<AnyData>>,
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
    fn add(&mut self, op: Command);

    fn gen(&self, range: Range<i64>) -> Vec<Vec<Box<dyn IVisData>>>;

    fn len(&self) -> usize;
}

pub trait IVisData: DynClone {
    // fn new(x: AnyData, y: AnyData, data: AnyData) -> Self;

    fn pos(&self) -> [AnyData; 2];

    fn data(&self) -> AnyData {
        AnyData::new(())
    }

    fn is_same(&self, another_data: &dyn IVisData) -> bool;

    fn matrix(&self, matrix: [[f64; 3]; 3]) -> AnyData;
}

dyn_clone::clone_trait_object!(IVisData);

pub trait IVisualizer {
    fn new(transform: [[f64; 3]; 3]) -> Self;
    fn plot(&self, ui: &mut egui::Ui, input: Vec<Vec<Box<dyn IVisData>>>, has_error: bool, show_inter_dash: bool, colorful_block: bool);

    fn transform(&mut self, matrix: [[f64; 3]; 3]);
}

pub trait ICodeEditor {
    type DST: IDataSource;

    fn show(&self, ui: &mut egui::Ui, code: &mut Self::DST, format: &dyn ICommandSyntax) -> egui::Response;
}
