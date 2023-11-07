use eframe::egui::{Response, Ui};
use crate::interfaces::{ICodeEditor};
use egui_code_editor::{CodeEditor as ECodeEditor, ColorTheme};
use crate::any_data::AnyData;
use crate::interfaces::ICommandSyntax;

pub struct CodeEditor {}

impl ICodeEditor for CodeEditor {

    fn show(&self, ui: &mut Ui, code: &mut AnyData, format: &dyn ICommandSyntax) -> Response {
        ECodeEditor::default()
            .id_source("code editor")
            .with_rows(12)
            .with_fontsize(14.0)
            .with_theme(ColorTheme::SONOKAI)
            .with_syntax(format.syntax())
            .with_numlines(true)
            .show(ui, code.cast_mut()).response
    }
}
