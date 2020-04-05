use super::arithmetic;
use super::error;
use super::fors;
use super::function;
use super::variable;
use super::super::ast::parsing;
use super::super::ast::asts;
use super::super::lexer::lexers;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::env;

pub fn run(root: asts::ExprAST) {
    let mut index = 0;
    let len = root.node.len();
    let mut variable = variable::Variable::new();
    let mut function = function::Function::new();
    function.push(root.node.clone());

    loop {
        if index >= len {
            break;
        }

        let node = &root.node[index];
        run_judg(node, &mut variable, &mut function);
        index += 1;
    }
}

fn if_run(
    result: &asts::Types,
    ifs: &Vec<asts::Types>,
    vec_variable: &mut variable::Variable,
    vec_function: &mut function::Function,
) -> Option<asts::Types> {
    match result {
        asts::Types::Boolean(boolean) => {
            if boolean.boolean {
                let (_, result) = scope(ifs, vec_variable, vec_function);

                return result;
            }
        }
        _ => {}
    }

    return None;
}

pub fn calculation(
    ast: &asts::Types,
    vec_variable: &mut variable::Variable,
    vec_function: &mut function::Function,
) -> asts::Types {
    match ast.clone() {
        asts::Types::Binary(mut binary) => {
            let vec_binary = vec_variable.variables_allocation(binary.node, vec_function);
            binary.node = vec_binary;
            return arithmetic::common(binary);
        }

        _ => {
            return ast.clone();
        }
    }
}

pub fn run_judg(
    node: &asts::Types,
    vec_variable: &mut variable::Variable,
    vec_function: &mut function::Function,
) -> (bool, Option<asts::Types>) {
    match node {
        asts::Types::Call(function) => {
            let result = vec_function.function_run(function, vec_variable);
            match result {
                Some(_) => {
                    return (false, result);
                }

                None => {
                    return (true, None);
                }
            }
        }

        asts::Types::Binary(bin) => {
            let result = calculation(node, vec_variable, vec_function);
            let mut is_continue = false;
            if bin.node.len() == 2 {
                match &bin.node[1] {
                    asts::Types::Binary(inner_bin) => {
                        if bin.op == inner_bin.op {
                            is_continue = true;
                        }
                    }

                    _ => {}
                }
            }

            if is_continue {
                match bin.node[0].clone() {
                    asts::Types::Variable(mut var) => {
                        var.node.push(result);
                        let varibles = asts::Types::Variable(var);
                        vec_variable.push(varibles);
                    }
                    _ => {}
                }
            }
        }

        asts::Types::Variable(var) => {
            let var_contents = vec_variable.variable(var, vec_function);
            let mut var_ast = asts::VariableAST::new(&var.name);
            match var_contents {
                Some(content) => {
                    var_ast.node.push(content);
                    vec_variable.push(asts::Types::Variable(var_ast));
                }

                None => {}
            }
        }

        asts::Types::If(ifs) => {
            let result = calculation(&ifs.judge[0], vec_variable, vec_function);
            let mut result_if = None;
            vec_variable.vec_push();
            vec_function.vec_push();
            if !ifs.node.is_empty() {
                vec_function.push(ifs.node.clone());
                result_if = if_run(&result, &ifs.node, vec_variable, vec_function);
            }
            vec_variable.last_remove();
            vec_function.last_remove();

            match result_if {
                Some(_) => {
                    return (false, result_if);
                }

                None => {
                    return (true, None);
                }
            }
        }

        asts::Types::For(fors) => {
            vec_variable.vec_push();
            vec_function.vec_push();
            vec_function.push(fors.node.clone());
            let result = fors::for_run(&fors.init_var, &fors.node, vec_variable, vec_function);
            vec_variable.last_remove();
            vec_function.last_remove();

            match result {
                Some(_) => {
                    return (false, result);
                }

                None => {
                    return (true, None);
                }
            }
        }

        asts::Types::Retrun(ret) => {
            if ret.node.is_empty() {
                return (false, None);
            }

            match ret.node[0].clone() {
                asts::Types::Binary(_) => {
                    let result = calculation(&ret.node[0], vec_variable, vec_function);
                    return (false, Some(result));
                }

                asts::Types::Variable(var) => {
                    let result = vec_variable.serch_variable(&var, vec_function);
                    return (false, Some(result[0].clone()));
                }

                asts::Types::Number(_) => {
                    let result = ret.node[0].clone();
                    return (false, Some(result));
                }

                _ => {
                    let err = error::Error::new(&ret.node[0]);
                    err.exit("retrun error");
                }
            }
        }

        asts::Types::Import(import) => {
            let result = calculation(&import.path[0], vec_variable, vec_function);
            let mut path = String::new();

            match result {
                asts::Types::Strings(string) => {
                    path = string.name;
                }
                _ => {
                    let err = error::Error::new(&result);
                    err.exit("import error");
                }
            }

            let result = read_file(&path);
            let functions = vec_function.retrun_insert(result.node.clone());
            let name:Vec<&str> = path.split('.').collect();
            let name:Vec<&str> = name[name.len() - 2].split('/').collect();
            let mut var_ast = asts::VariableAST::new(name[name.len() - 1]);
            var_ast.node = functions;
            vec_variable.push(asts::Types::Variable(var_ast));
        }

        _ => {}
    }

    return (true, None);
}

pub fn scope(
    ast: &Vec<asts::Types>,
    vec_variable: &mut variable::Variable,
    vec_function: &mut function::Function,
) -> (bool, Option<asts::Types>) {
    let mut index = 0;
    let len = ast.len();

    loop {
        if index >= len {
            break;
        }
        let node = &ast[index];
        let (is_continue, result) = run_judg(node, vec_variable, vec_function);
        if !is_continue {
            return (false, result);
        }
        index += 1;
    }

    return (true, None);
}

fn read_file(path:&str) -> asts::ExprAST{
    let relative_path = Path::new(path);
    let pwd = env::current_dir().unwrap();
    let absolute_path = pwd.join(relative_path);
    let file = File::open(absolute_path ).expect("file not found");
    let mut file_buffer = BufReader::new(&file);
    let mut content = String::new();
    file_buffer
        .read_to_string(&mut content)
        .expect("file not found");

    let mut lexer = lexers::Lexer::new(&content);
    let tokens = lexer.start();

    let mut pars = parsing::Parsing::new(&tokens);
    let result = pars.parsing();
    return result;
}