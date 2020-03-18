use super::super::ast::Ast;
use super::Arithmetic;
use super::For;

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

fn variables_allocation(serch: Vec<Ast::Types>, variable: &Vec<Ast::Types>) -> Vec<Ast::Types> {
    let mut ast_vec: Vec<Ast::Types> = Vec::new();
    for node in serch {
        match node {
            Ast::Types::Binary(mut bin) => {
                if !bin.node.is_empty() {
                    let vec = variables_allocation(bin.node.clone(), variable);
                    bin.node = vec;
                }
                ast_vec.push(Ast::Types::Binary(bin));
            }

            Ast::Types::Number(mut num) => {
                if !num.node.is_empty() {
                    let vec = variables_allocation(num.node.clone(), variable);
                    num.node = vec;
                }
                ast_vec.push(Ast::Types::Number(num));
            }

            Ast::Types::Variabel(var) => {
                let mut vec: Vec<Ast::Types> = Vec::new();

                if !var.node.is_empty() {
                    vec = variables_allocation(var.node.clone(), variable);
                }

                serch_variable(variable, &var.name);
            }
            _ => {}
        }
    }
    return ast_vec;
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

pub fn calculation(ast: Ast::Types) -> Ast::Types {
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
    let mut variable = String::new();
    for ast in ast_vec {
        match ast {
            Ast::Types::Variabel(var) => {
                if var.name == serch_word.to_string() {
                    match var.node[0].clone() {
                        Ast::Types::Strings(strings) => {
                            variable = strings.name.clone();
                        }

                        Ast::Types::Number(num) => {
                            variable = num.val.to_string();
                        }

                        Ast::Types::Variabel(var) => {
                            let var_name = var.name;
                            variable = serch_variable(&ast_vec, &var_name);
                        }

                        Ast::Types::Boolean(bools) => {
                            if bools.boolean {
                                variable = "true".to_string();
                            } else {
                                variable = "false".to_string();
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    return variable;
}

pub fn run_judg(node: &Ast::Types, vec_variable: &mut Vec<Ast::Types>) {
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
            For::for_run(&fors.init_var, &fors.node, vec_variable);
        }
        _ => {}
    }
}
