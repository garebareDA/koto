use super::super::ast::Ast;
use super::Arithmetic;

pub fn run(root: Ast::ExprAST) {
    let mut index = 0;
    let len = root.node.len();
    let mut vec_variable:Vec<Ast::Types> = Vec::new();

    loop {
        if index >= len {
            break;
        }

        let node = &root.node[index];
        run_judg(node, &mut vec_variable);
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

fn function_run(call_ast: &Ast::CallAST, variable: &Vec<Ast::Types>) {
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

            Ast::Types::Variabel(var) => {
                let result = serch_variable(variable, &var.name);
                println!("{}", result);
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
            return Ast::Types::Error(Ast::ErrorAST::new("Binary Error"));
        }
    }
}

fn serch_variable(ast_vec: &Vec<Ast::Types>, serch_word: &str) -> String {
    for ast in ast_vec {
        match ast {
            Ast::Types::Variabel(var) => {
                if var.name == serch_word.to_string() {
                    match var.node[0].clone() {
                        Ast::Types::Strings(strings) => {
                            return strings.name.clone();
                        }

                        Ast::Types::Number(num) => {
                            return num.val.to_string();
                        }

                        Ast::Types::Variabel(var) => {
                            let var_name = var.name;
                            return serch_variable(&ast_vec, &var_name);
                        }

                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    return "error".to_string();
}

fn run_judg (node: &Ast::Types, vec_variable:&mut Vec<Ast::Types>) {
    match node {
        Ast::Types::Call(function) => {
            function_run(function, &vec_variable);
        },

        Ast::Types::Variabel(var) => {
            let var_contents = variable(var.node[0].clone());
            let mut var_ast = Ast::VariableAST::new(&var.name);
            var_ast.node.push(var_contents);
            vec_variable.push(Ast::Types::Variabel(var_ast));
        },

        Ast::Types::If(ifs) => {
            //TODO ifの実行を実装
            
        }
        _ => {}
    }
}