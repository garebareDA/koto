use super::super::lexer::Token;
use super::Ast;

//構文解析
pub fn parsing(tokens: Vec<Token::TokenValue>) {
    let mut index = 0;
    let node = Ast::ExprAST::new();

    loop {
        let token = tokens[index].token;

        index += 1;
    }
}

fn judge(token: i64, string: &str, mut root: Ast::ExprAST) {
    if token == -6 {
        let print = Ast::CallAST::new("print");
        let call = Ast::Types::Call(print);
        root.node.push(call);
    }
}