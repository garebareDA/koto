use super::super::ast::Ast;
use super::Variable;

pub fn function_run(call_ast: &Ast::CallAST, variable: &mut Variable::Variable) {
    let callee = call_ast.callee.clone();
    if callee == "print" {
        let value = &call_ast.node[0];
        match value {
            Ast::Types::Variable(var) => {
                let var_result = variable.serch_variable(&var.name);
                print_var(&var_result);
            }
            _ => {print_var(&value);}
        }
    }

    
}

fn print_var(var_result: &Ast::Types){
    match var_result {
        Ast::Types::Strings(value) => {
            println!("{}", value.name);
        }

        Ast::Types::Number(number) => {
            println!("{}", number.val);
        }

        Ast::Types::Boolean(bools) => {
            println!("{}", bools.boolean);
        }

        _ => {}
    }
}