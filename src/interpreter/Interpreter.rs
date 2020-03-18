use super::super::ast::Ast;
use super::Arithmetic;
use super::For;
use super::Variable;
use super::Function;


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

pub fn calculation(ast: Ast::Types, variable: &Vec<Ast::Types>) -> Ast::Types {
    match ast {
        Ast::Types::Binary(mut binary) => {
            let vec_binary = Variable::variables_allocation(binary.node, variable);
            binary.node = vec_binary;
            return Arithmetic::common(binary);
        }
        _ => {
            return Ast::Types::Error(Ast::ErrorAST::new("Binary Error"));
        }
    }
}

pub fn run_judg(node: &Ast::Types, vec_variable: &mut Vec<Ast::Types>) {
    match node {
        Ast::Types::Call(function) => {
            Function::function_run(function, &vec_variable);
        }

        Ast::Types::Variabel(var) => {
            let var_contents = Variable::variable(var.node[0].clone(), vec_variable);
            let mut var_ast = Ast::VariableAST::new(&var.name);
            var_ast.node.push(var_contents);
            vec_variable.push(Ast::Types::Variabel(var_ast));
        }

        Ast::Types::If(ifs) => {
            let result = calculation(ifs.judge[0].clone(), vec_variable);
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