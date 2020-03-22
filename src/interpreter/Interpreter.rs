use super::super::ast::Ast;
use super::Arithmetic;
use super::For;
use super::Function;
use super::Variable;

pub fn run(root: Ast::ExprAST) {
    let mut index = 0;
    let len = root.node.len();
    let mut variable = Variable::Variable::new();
    let mut function = Function::function::new();
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
    result: &Ast::Types,
    ifs: &Vec<Ast::Types>,
    vec_variable: &mut Variable::Variable,
    vec_function: &mut Function::function,
) -> Option<Ast::Types> {
    match result {
        Ast::Types::Boolean(boolean) => {
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
    ast: Ast::Types,
    vec_variable: &mut Variable::Variable,
    vec_function: &mut Function::function,
) -> Ast::Types {
    match ast {
        Ast::Types::Binary(mut binary) => {
            let vec_binary = vec_variable.variables_allocation(binary.node, vec_function);
            binary.node = vec_binary;
            return Arithmetic::common(binary);
        }

        _ => {
            return Ast::Types::Error(Ast::ErrorAST::new("Binary Error"));
        }
    }
}

pub fn run_judg(
    node: &Ast::Types,
    vec_variable: &mut Variable::Variable,
    vec_function: &mut Function::function,
) -> (bool, Option<Ast::Types>) {
    match node {
        Ast::Types::Call(function) => {
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

        Ast::Types::Variable(var) => {
            let var_contents = vec_variable.variable(var.node[0].clone(), vec_function);
            let mut var_ast = Ast::VariableAST::new(&var.name);
            var_ast.node.push(var_contents);
            vec_variable.push(Ast::Types::Variable(var_ast));
        }

        Ast::Types::If(ifs) => {
            let result = calculation(ifs.judge[0].clone(), vec_variable, vec_function);
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

        Ast::Types::For(fors) => {
            vec_variable.vec_push();
            vec_function.vec_push();
            vec_function.push(fors.node.clone());
            let result = For::for_run(&fors.init_var, &fors.node, vec_variable, vec_function);
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

        Ast::Types::Retrun(ret) => {
            if ret.node.is_empty() {
                return (false, None);
            }

            match ret.node[0].clone() {
                Ast::Types::Binary(_) => {
                    let result = calculation(ret.node[0].clone(), vec_variable, vec_function);
                    return (false, Some(result));
                }

                Ast::Types::Variable(var) => {
                    let result = vec_variable.serch_variable(&var.name, vec_function);
                    return (false, Some(result));
                }

                _ => {}
            }
        }

        _ => {}
    }

    return (true, None);
}

pub fn scope(
    ast: &Vec<Ast::Types>,
    vec_variable: &mut Variable::Variable,
    vec_function: &mut Function::function,
) -> (bool, Option<Ast::Types>) {
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
