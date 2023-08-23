use std::ops::DerefMut;
use eframe::egui::{Response, Ui};
use crate::common_vec_op::TextDataSrc;
use crate::interfaces::{ICodeEditor, IDataSource};
use crate::syntax::CommonVecOpSyntax;
use egui_code_editor::{CodeEditor as ECodeEditor, ColorTheme};
use crate::interfaces::ICommandSyntax;

pub struct CodeEditor {}

impl ICodeEditor<String, TextDataSrc, CommonVecOpSyntax> for CodeEditor {
    fn show(&self, ui: &mut Ui, code: &mut TextDataSrc, format: CommonVecOpSyntax) -> Response {
        ECodeEditor::default()
            .id_source("code editor")
            .with_rows(12)
            .with_fontsize(14.0)
            .with_theme(ColorTheme::SONOKAI)
            .with_syntax(format.syntax())
            .with_numlines(true)
            .show(ui, code.get_ref("").unwrap().deref_mut())
    }
}
