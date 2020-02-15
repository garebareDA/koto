use super::super::lexer::Token;
use super::Ast;

//構文解析
pub fn parsing(mut tokens: Vec<Token::TokenValue>) {
    let mut root = Ast::ExprAST::new();

    loop {
        let token = tokens[0].token;
        let val = &tokens[0].val;
        let result = judge(token, val);

        root.node.push(result);
        tokens.remove(0);

        if tokens.is_empty(){
            break;
        }
    }
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