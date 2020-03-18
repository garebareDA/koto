use super::super::ast::Ast;
use super::Interpreter;


pub fn variable(variable: Ast::Types) -> Ast::Types {
    match variable {
        Ast::Types::Binary(_) => {
            return Interpreter::calculation(variable);
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

                let serch_result = serch_variable(variable, &var.name);
            }
            _ => {}
        }
    }
    return ast_vec;
}

pub fn serch_variable(ast_vec: &Vec<Ast::Types>, serch_word: &str) -> Ast::Types {
    let mut variable = Ast::Types::Error(Ast::ErrorAST::new("Vairable Error"));
    for ast in ast_vec {
        match ast {
            Ast::Types::Variabel(var) => {
                if var.name == serch_word.to_string() {
                    match var.node[0].clone() {

                        Ast::Types::Variabel(var) => {
                            let var_name = var.name;
                            variable = serch_variable(&ast_vec, &var_name);
                        }

                        _ => {variable = var.node[0].clone();}
                    }
                }
            }
            _ => {}
        }
    }

    return variable;
}