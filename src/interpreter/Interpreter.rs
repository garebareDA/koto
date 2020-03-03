use super::super::ast::Ast;
use super::Arithmetic;

pub fn run(root: Ast::ExprAST) {
    let mut index = 0;
    let len = root.node.len();
    loop {
        if index >= len {
            break;
        }

        let node = &root.node[index];
        match node {
            Ast::Types::Call(function) => {
                function_run(function);
            },

            Ast::Types::Variabel(var) => {
                let var_contents = variable(var.node[0].clone());
                println!("{:?}", var_contents);
                println!("{}", var.name);
                
            },
            _ => {}
        }

        index += 1;
    }
}

fn variable(variable: Ast::Types) -> Ast::Types {
    match variable {
        Ast::Types::Binary(_) => {
            return calculation(variable);
        },

        _ => {return variable;}
    }
}

fn function_run(call_ast: &Ast::CallAST) {
    let callee = call_ast.callee.clone();
    if callee == "print" {
        let value = &call_ast.node[0];
        match value {
            Ast::Types::Strings(value) => {
                println!("{}", value.name);
            }

            Ast::Types::Number(number) => {
                println!("{}", number.val);
            }
            _ => {}
        }
    }
}

fn calculation(ast: Ast::Types) -> Ast::Types {
    match ast {
        Ast::Types::Binary(binary) => {
            return Arithmetic::common(binary);
        },
        _ => {
            //TODO エラーのenumを返す
            return Ast::Types::Binary(Ast::BinaryAST::new('+'))
        }
    }
}