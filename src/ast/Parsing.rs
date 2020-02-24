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

        if token == 40 || token == 41 {
            tokens.remove(0);
            continue;
        }

        let mut result = judge(tokens);

        match result {
            Ast::Types::Call(mut function) => {
               let result_call = function_call(tokens);
               function.node.push(result_call);
               result = Ast::Types::Call(function);
            },

            Ast::Types::Number(_) => {
                result = calculation(tokens);
            },

            Ast::Types::Variabel(mut var) => {
                let result_var = variable(tokens);
                var.node.push(result_var);
                result = Ast::Types::Variabel(var);
            }

            Ast::Types::End(_) => {
                tokens.remove(0);
                continue;
            },

            _ => {}
        }

        root.node.push(result);

        tokens.remove(0);
    }

    return root;
}

fn judge(tokens: &mut Vec<Token::TokenValue>)-> Ast::Types {
    let token = tokens[0].token;
    let string = tokens[0].val.clone();

    if token == -6 {
        let print = Ast::CallAST::new("print");
        let call = Ast::Types::Call(print);
        return call;
    }

    if token == -7 {
        let string = Ast::StringAST::new(&string);
        let variable = Ast::Types::Strings(string);
        return variable;
    }

    if token == -8 {
        let num = string.parse().unwrap();
        let num = Ast::NumberAST::new(num);
        let number = Ast::Types::Number(num);
        return number;
    }

    if token == 43 || token == 45 || token == 47 || token == 37 || token == 42{
        let bin = Ast::BinaryAST::new(string.parse().unwrap());
        let binary = Ast::Types::Binary(bin);
        return binary;
    }

    if token == 59 {
        let end = Ast::EndAST::new();
        let end = Ast::Types::End(end);
        return end;
    }

    if token == -11 {
        tokens.remove(0);
        let string = tokens[0].val.clone();
        let variable = Ast::VariableAST::new(&string);
        let variable = Ast::Types::Variabel(variable);
        tokens.remove(0);
        return variable;
    }

    let variable = Ast::VariableAST::new(&string);
    let variable = Ast::Types::Variabel(variable);
    tokens.remove(0);
    return variable;
}

fn function_call(tokens: &mut Vec<Token::TokenValue>) -> Ast::Types {
    let string = Ast::StringAST::new("");
    let mut result = Ast::Types::Strings(string);

    loop {
        let token = tokens[0].token;

        if token == 40{
            tokens.remove(0);
            continue;
        }

        if token == 41{
            break;
        }

        result = judge(tokens);
        tokens.remove(0);
    }

    return result;
}

fn calculation(tokens: &mut Vec<Token::TokenValue>) -> Ast::Types {
    let mut number_vector:Vec<Ast::Types> = Vec::new();
    let mut binary_vector:Vec<Ast::Types> = Vec::new();

    loop {
        let result = judge(tokens);

        match result {
            Ast::Types::Binary(_) => {
                binary_vector.push(result);
            },

            Ast::Types::Number(_) => {
                number_vector.push(result);
            },

            Ast::Types::Strings(_) => {
                number_vector.push(result);
            }

            Ast::Types::End(_) => {
                break;
            },

            _ =>{}
        }

        tokens.remove(0);
    }

    if number_vector.len() == 1 {
        let number = number_vector[0].clone();
        return number;
    }

    number_vector.reverse();
    binary_vector.reverse();

    let mut index = 0;
    let mut ast_temp = Ast::Types::Binary(Ast::BinaryAST::new('+'));

    for binary in binary_vector {
       let mut number = number_vector[index].clone();

        if index > 0 {
            match number{
                Ast::Types::Number(mut numbers) => {
                    numbers.node.push(ast_temp.clone());
                    number = Ast::Types::Number(numbers);
                }
                _ => {}
            }
        }

        match binary {
            Ast::Types::Binary(mut binary) => {
                binary.node.push(number.clone());
                ast_temp = Ast::Types::Binary(binary);
            }
            _ => {}
        }

        index += 1;
    }

    match ast_temp {
        Ast::Types::Binary(mut binary) => {
            binary.node.push(number_vector[index].clone());
            binary.node.reverse();
            let ast_binary = Ast::Types::Binary(binary);
            return ast_binary;
        }

        _ => { return Ast::Types::Binary(Ast::BinaryAST::new('+'));}
    }
}

fn variable(tokens: &mut Vec<Token::TokenValue>) -> Ast::Types {
    let token = tokens[0].token;

    if token == 40 || token == 41 {
        tokens.remove(0);
    }

    let mut result = judge(tokens);

    match result {
        Ast::Types::Number(_) => {
           result = calculation(tokens);
        },

        _ => {}
    }

    return result;
}