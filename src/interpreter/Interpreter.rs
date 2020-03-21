use super::super::ast::Ast;
use super::Arithmetic;
use super::For;
use super::Variable;
use super::Function;

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

fn if_run(result: &Ast::Types, ifs: &Vec<Ast::Types>, vec_variable: &mut Variable::Variable, mut vec_function: &mut Function::function) {
    match result {
        Ast::Types::Boolean(boolean) => {
            if boolean.boolean {
                scope(ifs, vec_variable, vec_function);
            }
        }
        _ => {}
    }
}

pub fn calculation(ast: Ast::Types, variable: &mut Variable::Variable) -> Ast::Types {
    match ast {
        Ast::Types::Binary(mut binary) => {
            let vec_binary = variable.variables_allocation(binary.node);
            binary.node = vec_binary;
            return Arithmetic::common(binary);
        }

        _ => {
            return Ast::Types::Error(Ast::ErrorAST::new("Binary Error"));
        }
    }
}

pub fn run_judg(node: &Ast::Types, vec_variable: &mut Variable::Variable, vec_function: &mut Function::function) -> bool {
    match node {
        Ast::Types::Call(function) => {
            vec_function.function_run(function, vec_variable);
        }

        Ast::Types::Variable(var) => {
            let var_contents = vec_variable.variable(var.node[0].clone());
            let mut var_ast = Ast::VariableAST::new(&var.name);
            var_ast.node.push(var_contents);
            vec_variable.push(Ast::Types::Variable(var_ast));
        }

        Ast::Types::If(ifs) => {
            let result = calculation(ifs.judge[0].clone(), vec_variable);
            vec_variable.vec_push();
            vec_function.vec_push();
            if !ifs.node.is_empty() {
                vec_function.push(ifs.node.clone());
                if_run(&result, &ifs.node, vec_variable, vec_function);
            }
            vec_variable.last_remove();
            vec_function.last_remove();
        }

        Ast::Types::For(fors) => {
            vec_variable.vec_push();
            vec_function.vec_push();
            vec_function.push(fors.node.clone());
            For::for_run(&fors.init_var, &fors.node, vec_variable, vec_function);
            vec_variable.last_remove();
            vec_function.last_remove();
        }

        Ast::Types::Retrun(_) => {
            return false;
        }

        _ => {}
    }

    return true;
}

pub fn scope(ast: &Vec<Ast::Types>, vec_variable: &mut Variable::Variable, mut vec_function: &mut Function::function) -> bool {
    let mut index = 0;
    let len = ast.len();

    loop {
        if index >= len {
            break;
        }
        let node = &ast[index];
        let is_continue = run_judg(node, vec_variable, vec_function);
        if !is_continue {
            return false;
        }
        index += 1;
    }

    return true;
}