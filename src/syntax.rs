use crate::interfaces::{CommandDescription, ICommandSyntax};

pub struct CommonVecOpSyntax {}

impl ICommandSyntax for CommonVecOpSyntax {
    fn name(&self) -> &'static str {
        "CommonVecOpSyntax"
    }

    fn formats(&self) -> Vec<CommandDescription> {
        return vec![
            CommandDescription { name: "MOVE", argc: 2 },
            CommandDescription { name: "LINE", argc: 2 },
            CommandDescription { name: "QUAD", argc: 4 },
            CommandDescription { name: "CUBI", argc: 6 },
            CommandDescription { name: "END", argc: 0 },
        ];
    }
}
