use super::super::ast::Ast;
use std::process::exit;

pub struct Error {
    pub error:Ast::Types,
}

impl Error {
    pub fn new(ast: &Ast::Types) -> Error {
        let ast = ast.clone();
        Error {
            error: ast,
        }
    }

    pub fn exit(self) {
        self.error_message();
        exit(0);
    }

    fn error_message(self) {
        match self.error.clone() {
            Ast::Types::Binary(bin) => {
                eprintln!("{}",bin.op);
            }

            Ast::Types::Strings(string) => {
                eprintln!("{}",string.name);
            }

            Ast::Types::Vector(vec) => {
                eprintln!("{:?}",vec.node);
            }

            Ast::Types::Number(num) => {
                eprintln!("{:?}", num.val);
            }

            Ast::Types::Variable(var) => {
                eprintln!("{:?}", var.name);
            }

            Ast::Types::If(ifs) => {
                eprintln!("{:?}", ifs.judge);
            }

            Ast::Types::For(fors) => {
                eprintln!("{:?}", fors.init_var);
            }

            Ast::Types::Error(err) => {
                eprintln!("{}", err.error);
            }

            _ => {}
        }
    }
}