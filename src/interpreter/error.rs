use super::super::ast::asts;
use std::process::exit;

pub struct Error {
    pub error:asts::Types,
}

impl Error {
    pub fn new(ast: &asts::Types) -> Error {
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
            asts::Types::Binary(bin) => {
                eprintln!("{} {}",bin.op, error_message);
            }

            asts::Types::Strings(string) => {
                eprintln!("{} {}",string.name, error_message);
            }

            asts::Types::Vector(vec) => {
                eprintln!("{:?} {}",vec.node, error_message);
            }

            asts::Types::Number(num) => {
                eprintln!("{} {:?}", num.val, error_message);
            }

            asts::Types::Variable(var) => {
                eprintln!("{:?} {}", var.name, error_message);
            }

            asts::Types::If(ifs) => {
                eprintln!("{:?} {}", ifs.judge, error_message);
            }

            asts::Types::For(fors) => {
                eprintln!("{:?} {}", fors.init_var, error_message);
            }

            asts::Types::Error(err) => {
                eprintln!("{} {}", err.error, error_message);
            }

            _ => {}
        }
    }
}