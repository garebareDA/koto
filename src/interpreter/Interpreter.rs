use super::super::ast::Ast;

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
                variable(var.node[0].clone());
            },
            _ => {}
        }

        index += 1;
    }
}

fn variable(variable: Ast::Types) {
    match variable {
        Ast::Types::Binary(_) => {
            calculation(variable);
        },

        Ast::Types::Number(num) => {
            println!("{:?}", num);
        },

        Ast::Types::Strings(strings) => {
            println!("{:?}", strings);
        },

        _ => {}
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

fn calculation(ast: Ast::Types) {
    match ast {
        Ast::Types::Binary(binary) => {
            let op = binary.op;
            println!("{:?}", binary);
            if op == '%'{

            }

            if op == '*' {

            }

            if op == '/'{

            }

            if op == '-' {

            }

            if op == '+' {
                match binary.node[1].clone() {
                    Ast::Types::Number(num) => {
                        
                    }
                    _ => {}
                }
            }

        },

        _ => {}
    }
}