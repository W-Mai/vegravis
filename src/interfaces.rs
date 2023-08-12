use std::ops::Range;
use eframe::egui;
use eframe::emath::Numeric;

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

pub trait IParser<PT: Numeric, ST, CT: Numeric, VDT: IVisData<PT>, DST: IDataSource<ST>, G: IVisDataGenerator<CT, PT, VDT>> {
    fn parse(&self, input: DST) -> Result<G, String>;
}

pub trait IEncoder {
    fn encode(&self, input: &str) -> String;
}

pub trait IVisDataGenerator<CT: Numeric, PT: Numeric, VDT: IVisData<PT>> {
    fn add(&mut self, op: Command<CT>);

    fn gen(&self, range: Range<i64>) -> Vec<VDT>;
}

pub trait IVisData<PT: Numeric> {
    fn new(x: PT, y: PT) -> Self;

    fn pos(&self) -> (PT, PT);
}

pub trait IVisualizer<PT: Numeric, VDT: IVisData<PT>> {
    fn plot(&self, input: VDT);
}

pub trait ICodeEditor<ST, DST: IDataSource<ST>, CST: ICommandSyntax> {
    fn show(&self, ui: &mut egui::Ui, code: &mut DST, format: CST) -> egui::Response;
}
