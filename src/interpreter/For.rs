use super::super::ast::Ast;
use super::Variable;
use super::Function;
use super::Interpreter;
use super::Error;

//TODO 全体的にリファクタリング
pub fn for_run(ast_for: &Vec<Ast::Types>, ast: &Vec<Ast::Types>, vec_variable: &mut Variable::Variable, vec_function: &mut Function::function) -> Option<Ast::Types>{
    let variant = ast_for[0].clone();
    let judge = ast_for[1].clone();
    let loop_for = ast_for[2].clone();

    let mut result_var = Ast::Types::Error(Ast::ErrorAST::new("variable error"));
    let mut name = "".to_string();

    match variant {
        Ast::Types::Variable(var) => {
            name = var.name;
            match var.node[0].clone() {
                Ast::Types::Number(_) => {result_var = var.node[0].clone();}
                Ast::Types::Binary(_) => {result_var = Interpreter::calculation(&var.node[0], vec_variable, vec_function);}
                _ => {
                    let err = Error::Error::new(&var.node[0]);
                    err.exit("for init variable error");
                }
            }
        }

        _ => {
            let err = Error::Error::new(&variant);
            err.exit("fot init error");
        }
    }

    loop {
        match judge.clone() {
            Ast::Types::Binary(mut bin) => {
                let var = for_variables(&name, &result_var, bin.node);
                bin.node = var;
                let result = Interpreter::calculation(&Ast::Types::Binary(bin), vec_variable, vec_function);
                match result {
                    Ast::Types::Boolean(bools) => {
                        if bools.boolean {
                            break;
                        } else {
                            let mut var_ast = Ast::VariableAST::new(&name);
                            var_ast.node.push(result_var.clone());
                            vec_variable.push(Ast::Types::Variable(var_ast));
                            let (is_continue, result) = Interpreter::scope(ast, vec_variable, vec_function);
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
                        let err = Error::Error::new(&result);
                        err.exit("Binary error");
                    },
                }
            }
            _ => {
                let err = Error::Error::new(&judge);
                err.exit("for judg error");
            }
        }

        match loop_for.clone() {
            Ast::Types::Binary(mut bin) => {
                let result = for_variables(&name, &result_var, bin.node.clone());
                let innner_bin = bin.node[1].clone();
                match innner_bin {
                    Ast::Types::Binary(bin) => {
                        match result_var.clone() {
                            Ast::Types::Number(mut num) =>{
                                if bin.op == '+'{
                                    num.val += 1;
                                }else if bin.op == '-'{
                                    num.val -= 1;
                                }
                                result_var = Ast::Types::Number(num);
                                continue;
                            }
                            _ => {
                                let err = Error::Error::new(&result_var);
                                err.exit("for binary error");
                            }
                        }
                    }
                    _ => {
                        let err = Error::Error::new(&innner_bin);
                        err.exit("for binary error");
                    }
                }
                bin.node = result;
                result_var = Interpreter::calculation(&Ast::Types::Binary(bin), vec_variable, vec_function);
            }
            _ => {
                let err = Error::Error::new(&loop_for);
                err.exit("for binary error");
            }
        }
    }

    return None;
}

fn for_variables(name: &str, result: &Ast::Types, variables: Vec<Ast::Types>) -> Vec<Ast::Types> {
    let mut ast_vec: Vec<Ast::Types> = Vec::new();
    for node in variables {
        match node {
            Ast::Types::Binary(mut bin) => {
                if !bin.node.is_empty() {
                    let vec = for_variables(name, result, bin.node.clone());
                    bin.node = vec;
                }
                ast_vec.push(Ast::Types::Binary(bin));
            }

            Ast::Types::Number(mut num) => {
                if !num.node.is_empty() {
                    let vec = for_variables(name, result, num.node.clone());
                    num.node = vec;
                }
                ast_vec.push(Ast::Types::Number(num));
            }

            Ast::Types::Variable(var) => {
                let mut vec: Vec<Ast::Types> = Vec::new();

                if !var.node.is_empty() {
                    vec = for_variables(name, result, var.node.clone());
                }

                match result.clone() {
                    Ast::Types::Number(mut inner_num) => {
                        inner_num.node = vec;
                        ast_vec.push(Ast::Types::Number(inner_num));
                    }
                    _ => {
                        let err = Error::Error::new(&result);
                        err.exit("for variable error");
                    }
                }
            }
            _ => {}
        }
    }

    return ast_vec;
}