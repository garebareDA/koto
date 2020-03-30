use super::super::ast::asts;
use super::variable;
use super::function;
use super::interpreters;
use super::error;

pub fn for_run(ast_for: &Vec<asts::Types>, ast: &Vec<asts::Types>, vec_variable: &mut variable::Variable, vec_function: &mut function::Function) -> Option<asts::Types>{
    let variant = ast_for[0].clone();
    let judge = ast_for[1].clone();
    let loop_for = ast_for[2].clone();

    let mut result_var = asts::Types::Error(asts::ErrorAST::new("variable error"));
    let mut name = "".to_string();

    match variant {
        asts::Types::Variable(var) => {
            name = var.name;
            match var.node[0].clone() {
                asts::Types::Number(_) => {result_var = var.node[0].clone();}
                asts::Types::Binary(_) => {result_var = interpreters::calculation(&var.node[0], vec_variable, vec_function);}
                _ => {
                    let err = error::Error::new(&var.node[0]);
                    err.exit("for init variable error");
                }
            }
        }

        _ => {
            let err = error::Error::new(&variant);
            err.exit("fot init error");
        }
    }

    loop {
        match judge.clone() {
            asts::Types::Binary(mut bin) => {
                let var = for_variables(&name, &result_var, bin.node);
                bin.node = var;
                let result = interpreters::calculation(&asts::Types::Binary(bin), vec_variable, vec_function);
                match result {
                    asts::Types::Boolean(bools) => {
                        if bools.boolean {
                            break;
                        } else {
                            let mut var_ast = asts::VariableAST::new(&name);
                            var_ast.node.push(result_var.clone());
                            vec_variable.push(asts::Types::Variable(var_ast));
                            let (is_continue, result) = interpreters::scope(ast, vec_variable, vec_function);
                            if !is_continue {
                                match result {
                                    Some(_) => {
                                        return result;
                                    }
                                    None => {break;}
                                }
                            }
                        }
                    }
                    _ => {
                        let err = error::Error::new(&result);
                        err.exit("Binary error");
                    },
                }
            }
            _ => {
                let err = error::Error::new(&judge);
                err.exit("For judge error");
            }
        }

        result_var = interpreters::calculation(&loop_for, vec_variable, vec_function);
    }

    return None;
}

fn for_variables(name: &str, result: &asts::Types, variables: Vec<asts::Types>) -> Vec<asts::Types> {
    let mut ast_vec: Vec<asts::Types> = Vec::new();
    for node in variables {
        match node {
            asts::Types::Binary(mut bin) => {
                if !bin.node.is_empty() {
                    let vec = for_variables(name, result, bin.node.clone());
                    bin.node = vec;
                }
                ast_vec.push(asts::Types::Binary(bin));
            }

            asts::Types::Number(mut num) => {
                if !num.node.is_empty() {
                    let vec = for_variables(name, result, num.node.clone());
                    num.node = vec;
                }
                ast_vec.push(asts::Types::Number(num));
            }

            asts::Types::Variable(var) => {
                let mut vec: Vec<asts::Types> = Vec::new();

                if !var.node.is_empty() {
                    vec = for_variables(name, result, var.node.clone());
                }

                match result.clone() {
                    asts::Types::Number(mut inner_num) => {
                        inner_num.node = vec;
                        ast_vec.push(asts::Types::Number(inner_num));
                    }
                    _ => {
                        let err = error::Error::new(&result);
                        err.exit("for variable error");
                    }
                }
            }
            _ => {}
        }
    }

    return ast_vec;
}