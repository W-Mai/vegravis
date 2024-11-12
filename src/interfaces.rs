use crate::any_data::AnyData;
use dyn_clone::DynClone;
use eframe::egui;
use egui_code_editor::Syntax;
use levenshtein::levenshtein;
use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};
use std::ops::Range;
use std::rc::Rc;

pub trait ICommandDescription {
    fn name(&self) -> Vec<&str>;
    fn argc(&self) -> usize;

    fn operate(&self, ctx: &mut AnyData, argv: Rc<Vec<AnyData>>) -> Vec<AnyData>;
}

#[derive(Clone)]
pub struct Command {
    pub dsc: &'static dyn ICommandDescription,
    pub argv: Rc<Vec<AnyData>>,
}

impl Command {
    pub fn pack(&mut self, argv: Vec<AnyData>) {
        assert_eq!(argv.len(), self.dsc.argc());

        self.argv = Rc::new(argv);
    }

    pub fn operate(&self, ctx: &mut AnyData) -> Vec<AnyData> {
        self.dsc.operate(ctx, self.argv.clone())
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.dsc.name())
    }
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
        Self {
            msg: "Internal Error".to_owned(),
            cursor: Cursor::default(),
        }
    }
}

pub trait ICommandSyntax {
    fn name(&self) -> &'static str;

    fn case_sensitive(&self) -> bool {
        false
    }

    fn formats(&self) -> Vec<&'static dyn ICommandDescription>;
    fn syntax(&self) -> Syntax {
        let keywords = self
            .formats()
            .iter()
            .flat_map(|cmd| cmd.name())
            .collect::<BTreeSet<&str>>();
        let types = BTreeSet::new();
        let special = BTreeSet::new();

        Syntax {
            language: self.name(),
            case_sensitive: self.case_sensitive(),
            comment: "//",
            comment_multiline: ["/*", "*/"],
            hyperlinks: Default::default(),
            keywords,
            types,
            special,
        }
    }

    fn match_command(&self, cmd: &str) -> Result<Command, &str> {
        let cmd = cmd.to_owned();
        let cmd = if self.case_sensitive() {
            cmd
        } else {
            cmd.to_uppercase()
        };
        let cmd = cmd.as_str();
        for desc in self.formats() {
            if desc.name().contains(&cmd) {
                return Ok(Command {
                    dsc: desc,
                    argv: Rc::new(vec![]),
                });
            }
        }
        let mut dists = vec![];
        for desc in self.formats() {
            for name in desc.name() {
                dists.push((levenshtein(cmd, name), name));
            }
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

pub trait IParser<'a> {
    fn new(code: AnyData, gen: &'a mut dyn IVisDataGenerator) -> Self;

    fn parse(&'a mut self) -> Result<&'a mut dyn IVisDataGenerator, ParseError>;
}

#[allow(dead_code)]
pub trait IEncoder {
    fn encode(&self, input: &str) -> String;
}

/// Add data and generate VisData
/// Common usage is parse to a parser, then the parser call VisDataGenerator.add to add data.
/// Finally, pass the data added VisDataGenerator out of the parser
/// Then, get the VisData vis using the VisDataGenerator.gen method.
pub trait IVisDataGenerator {
    fn add(&mut self, op: Command);

    fn gen(&self, range: Range<i64>) -> Vec<Vec<Box<dyn IVisData>>>;

    fn len(&self) -> usize;

    fn command_syntax(&self) -> &'static dyn ICommandSyntax;
}

pub trait IVisData: DynClone {
    fn pos(&self) -> [AnyData; 2];

    fn data(&self) -> AnyData {
        AnyData::new(())
    }

    fn is_same(&self, another_data: &dyn IVisData) -> bool;

    fn matrix(&self, matrix: [[f64; 3]; 3]) -> AnyData;
}

dyn_clone::clone_trait_object!(IVisData);

#[allow(dead_code)]
pub trait IVisualizer {
    fn new(transform: [[f64; 3]; 3]) -> Self;
    fn plot(
        &self,
        ui: &mut egui::Ui,
        input: Vec<Vec<Box<dyn IVisData>>>,
        has_error: bool,
        show_inter_dash: bool,
        colorful_block: bool,
        lcd_coords: bool,
    );

    fn transform(&mut self, matrix: [[f64; 3]; 3]);
}

pub trait ICodeEditor {
    fn show(
        &self,
        ui: &mut egui::Ui,
        code: &mut AnyData,
        format: &dyn ICommandSyntax,
    ) -> egui::Response;
}
