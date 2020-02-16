use super::super::lexer::Token;
use super::Ast;

//構文解析
pub fn parsing(tokens: &mut Vec<Token::TokenValue>) -> Ast::ExprAST {
    let mut root = Ast::ExprAST::new();

    loop {
        if tokens.is_empty(){
            break;
        }

        let token = tokens[0].token;
        let val = &tokens[0].val;

        if token == 40 || token == 41 {
            tokens.remove(0);
            continue;
        }

        let mut result = judge(token, val);

        match result {
            Ast::Types::Call(mut function) => {
               let result_call = function_call(tokens);
               function.node.push(result_call);
               result = Ast::Types::Call(function);
            },
            _ => {}
        }

        root.node.push(result);
        tokens.remove(0);
    }

    return root;
}

fn judge(token: i64, string: &str,)-> Ast::Types {
    if token == -6 {
        let print = Ast::CallAST::new("print");
        let call = Ast::Types::Call(print);
        return call;
    }

    if token == -7 {
        let string = Ast::VariableAST::new(string);
        let variable = Ast::Types::Variable(string);
        return variable;
    }

    if token == -8 {
        let num = string.parse().unwrap();
        let num = Ast::NumberAST::new(num);
        let number = Ast::Types::Number(num);
        return number;
    }

    let string = Ast::VariableAST::new(string);
    let variable = Ast::Types::Variable(string);
    return variable;
}

fn function_call(tokens: &mut Vec<Token::TokenValue>) -> Ast::Types {
    let string = Ast::VariableAST::new("");
    let mut result = Ast::Types::Variable(string);

    loop {
        let token = tokens[0].token;
        let val = &tokens[0].val;

        if token == 40{
            tokens.remove(0);
            continue;
        }

        if token == 41{
            break;
        }

        result = judge(token, val);
        tokens.remove(0);
    }

    return result;
}