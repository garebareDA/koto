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

    pub fn exit(self, message: &str) {
        self.error_message(message);
        exit(0);
    }

    fn error_message(self, error_message:&str) {
        match self.error.clone() {
            Ast::Types::Binary(bin) => {
                eprintln!("{} {}",bin.op, error_message);
            }

            Ast::Types::Strings(string) => {
                eprintln!("{} {}",string.name, error_message);
            }

            Ast::Types::Vector(vec) => {
                eprintln!("{:?} {}",vec.node, error_message);
            }

            Ast::Types::Number(num) => {
                eprintln!("{} {:?}", num.val, error_message);
            }

            Ast::Types::Variable(var) => {
                eprintln!("{:?} {}", var.name, error_message);
            }

            Ast::Types::If(ifs) => {
                eprintln!("{:?} {}", ifs.judge, error_message);
            }

            Ast::Types::For(fors) => {
                eprintln!("{:?} {}", fors.init_var, error_message);
            }

            Ast::Types::Error(err) => {
                eprintln!("{} {}", err.error, error_message);
            }

            _ => {}
        }
    }
}