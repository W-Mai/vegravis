use std::ops::Range;
use eframe::egui;

pub struct CommandDescription {
    pub name: String,
    pub argc: usize,
}

pub struct Command<T> {
    pub dsc: CommandDescription,
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
    fn formats(&self) -> Vec<CommandDescription>;
}

pub trait IDataSource<ST> {
    fn get(&self, name: &str) -> Option<ST>;
}

pub trait IParser<ST, CT, VDT: IVisData, DST: IDataSource<ST>, G: IVisDataGenerator<CT, VDT>> {
    fn parse(&self, input: DST) -> Result<G, String>;
}

pub trait IEncoder {
    fn encode(&self, input: &str) -> String;
}

pub trait IVisDataGenerator<CT, VDT: IVisData> {
    fn add(&mut self, op: Command<CT>);

    fn gen(&self, range: Range<i64>) -> VDT;
}

pub trait IVisData {}

pub trait IVisualizer<VDT: IVisData> {
    fn plot(&self, input: VDT);
}

pub trait ICodeEditor<ST, DST: IDataSource<ST>, CST: ICommandSyntax> {
    fn show(&self, ui: &mut egui::Ui, code: &mut DST, format: CST) -> egui::Response;
}
