use super::super::ast::Ast;
use super::Arithmetic;

pub fn run(root: Ast::ExprAST) {
    let mut index = 0;
    let len = root.node.len();
    let mut vec_variable: Vec<Ast::Types> = Vec::new();

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
        }

        _ => {
            return variable;
        }
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

fn if_run(result: &Ast::Types, ifs: &Vec<Ast::Types>, vec_variable: &mut Vec<Ast::Types>) {
    match result {
        Ast::Types::Boolean(boolean) => {
            if boolean.boolean {
                let mut index = 0;
                let len = ifs.len();

                loop {
                    if index >= len {
                        break;
                    }
                    let node = &ifs[index];
                    run_judg(node, vec_variable);
                    index += 1;
                }
            }
        }
        _ => {}
    }
}

fn for_run(ast_for: &Vec<Ast::Types>) {
    let variant = ast_for[0].clone();
    let judge = ast_for[1].clone();
    let loop_for = ast_for[2].clone();

    println!("{:?}",judge);

    let mut result = Ast::Types::Error(Ast::ErrorAST::new("variable error"));
    let mut name = "".to_string();

    match variant {
        Ast::Types::Variabel(var) => {
            name = var.name;
            result = calculation(var.node[0].clone());
        }

        _ => {}
    }

    match judge.clone() {
        Ast::Types::Binary(bin) => {
            let a = for_variables(&name, result, bin.node);
        }

        _ => {}
    }

    match loop_for.clone() {
        Ast::Types::Binary(bin) => {}

        _ => {}
    }
}

fn for_variables(name: &str, result: Ast::Types, variables: Vec<Ast::Types>) {
    for node in variables {
        match node {
            Ast::Types::Binary(bin) => {
                if !bin.node.is_empty() {
                    for_variables(name, result.clone(), bin.node);
                }
            }

            Ast::Types::Number(num) => {
                if !num.node.is_empty() {
                    for_variables(name, result.clone(), num.node);
                }
            }

            Ast::Types::Variabel(var) => {
                if !var.node.is_empty() {
                    for_variables(name, result.clone(), var.node);
                }
            }

            _ => {}
        }
    }
}

fn calculation(ast: Ast::Types) -> Ast::Types {
    match ast {
        Ast::Types::Binary(binary) => {
            return Arithmetic::common(binary);
        }
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

                        Ast::Types::Boolean(bools) => {
                            if bools.boolean {
                                return "true".to_string();
                            } else {
                                return "false".to_string();
                            }
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

fn run_judg(node: &Ast::Types, vec_variable: &mut Vec<Ast::Types>) {
    match node {
        Ast::Types::Call(function) => {
            function_run(function, &vec_variable);
        }

        Ast::Types::Variabel(var) => {
            let var_contents = variable(var.node[0].clone());
            let mut var_ast = Ast::VariableAST::new(&var.name);
            var_ast.node.push(var_contents);
            vec_variable.push(Ast::Types::Variabel(var_ast));
        }

        Ast::Types::If(ifs) => {
            let result = calculation(ifs.judge[0].clone());
            if !ifs.node.is_empty() {
                if_run(&result, &ifs.node, vec_variable);
            }
        }

        Ast::Types::For(fors) => {
            for_run(&fors.init_var);
        }
        _ => {}
    }
}
