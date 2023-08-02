use egui_code_editor::Syntax;
use std::collections::HashSet;

pub fn vec_op_syntax() -> Syntax {
    Syntax {
        language: "VecOp",
        case_sensitive: false,
        comment: "//",
        comment_multiline: ["/*", "*/"],
        keywords: HashSet::from([
            "MOVE",
            "LINE",
            "QUAD",
            "CUBI"
        ]),
        types: HashSet::from([]),
        special: HashSet::from([]),
    }
}
