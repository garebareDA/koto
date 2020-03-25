use super::super::lexer::Token;
use super::Ast;

//構文解析
pub fn parsing(tokens: &mut Vec<Token::TokenValue>) -> Ast::ExprAST {
    let mut root = Ast::ExprAST::new();
    let result = syntax(tokens);
    root.node = result;
    return root;
}

fn judge(tokens: &mut Vec<Token::TokenValue>) -> Ast::Types {
    let mut token = tokens[0].token;
    let mut string = tokens[0].val.clone();

    if token == -9 {
        tokens.remove(0);
        token = tokens[0].token;
        string = tokens[0].val.clone();
    }

    if token == -1 {
        tokens.remove(0);
        let result = calculation(tokens);
        let mut result_vec: Vec<Ast::Types> = Vec::new();
        result_vec.push(result);
        let if_ast = Ast::IfAST::new(result_vec);
        return Ast::Types::If(if_ast);
    }

    if token == -4 {
        tokens.remove(0);

        let var = scope(tokens);
        tokens.remove(0);

        let result = calculation(tokens);
        tokens.remove(0);

        let loop_op = calculation(tokens);

        match var {
            Some(var) => {
                let for_ast = Ast::ForAST::new(var, result, loop_op);
                let for_types = Ast::Types::For(for_ast);
                return for_types;
            }

            None => {
                return Ast::Types::Error(Ast::ErrorAST::new("for parsing error"));
            }
        }
    }

    if token == -5 {
        tokens.remove(0);
        return function(tokens);
    }

    if token == -6 {
        let print = Ast::CallAST::new("print");
        let call = Ast::Types::Call(print);
        return call;
    }

    if token == -7 {
        let string = Ast::StringAST::new(&string);
        let strings = Ast::Types::Strings(string);
        return strings;
    }

    if token == -8 {
        let num = string.parse().unwrap();
        let num = Ast::NumberAST::new(num);
        let number = Ast::Types::Number(num);
        return number;
    }

    if token == -10 {
        if tokens[1].token == 40 {
            let call = Ast::CallAST::new(&string);
            let call = Ast::Types::Call(call);
            return call;
        }

        if tokens[1].token == 91 {
            let mut vec_tokens = tokens.clone();
            vec_tokens.remove(0);
            let index = calculation(&mut vec_tokens);

            let mut variable = Ast::VariableAST::new(&string);
            variable.index = Some(vec![index]);
            let variable = Ast::Types::Variable(variable);
            return variable;
        }

        let variable = Ast::VariableAST::new(&string);
        let variable = Ast::Types::Variable(variable);
        return variable;
    }

    if token == -11 {
        tokens.remove(0);
        let string = tokens[0].val.clone();
        let variable = Ast::VariableAST::new(&string);
        let variable = Ast::Types::Variable(variable);
        tokens.remove(0);
        tokens.remove(0);
        return variable;
    }

    if token == -12 {
        if string == "true" {
            let bools = Ast::BooleanAST::new(true);
            let bools = Ast::Types::Boolean(bools);
            return bools;
        } else if string == "false" {
            let bools = Ast::BooleanAST::new(false);
            let bools = Ast::Types::Boolean(bools);
            return bools;
        }
    }

    if token == -13 {
        tokens.remove(0);
        let mut retrun_ast = Ast::RetrunAST::new();
        if tokens[0].token == 59 {
            return Ast::Types::Retrun(retrun_ast);
        }
        let result = judge(tokens);
        retrun_ast.node.push(result);
        return Ast::Types::Retrun(retrun_ast);
    }

    if token == -14 {
        tokens.remove(0);
        let result = vector(tokens);
        let mut vector_ast = Ast::VectorAST::new();
        vector_ast.node = result;
        return Ast::Types::Vector(vector_ast);
    }

    if token == 40 || token == 41 {
        let parent = Ast::ParenthesesAST::new(string.chars().nth(0).unwrap());
        let parent = Ast::Types::Parent(parent);
        return parent;
    }

    if token == 44 {
        let comma = Ast::CommaAST::new(',');
        let comma = Ast::Types::Comma(comma);
        return comma;
    }

    if token == 43 || token == 45 || token == 47 || token == 37 || token == 42 {
        let bin = Ast::BinaryAST::new(string.parse().unwrap());
        let binary = Ast::Types::Binary(bin);
        return binary;
    }

    if token == 60 || token == 62 || token == 124 || token == 61 || token == 33 || token == 38 {
        let mut bin = Ast::BinaryAST::new(string.parse().unwrap());

        let token = tokens[1].token;

        if token == 61 || token == 38 || token == 124 {
            tokens.remove(0);
            let string = tokens[0].val.clone();
            let in_binary = Ast::BinaryAST::new(string.parse().unwrap());
            let in_binary = Ast::Types::Binary(in_binary);
            bin.node.push(in_binary);
        }

        let binary = Ast::Types::Binary(bin);
        return binary;
    }

    if token == 59 {
        let end = Ast::EndAST::new();
        let end = Ast::Types::End(end);
        return end;
    }

    if token == 91 || token == 93 {
        let squar = Ast::SquareAST::new(string.parse().unwrap());
        let squar = Ast::Types::Square(squar);
        return squar;
    }

    if token == 123 || token == 125 {
        let scope_ast = Ast::ScopeAST::new(string.parse().unwrap());
        let scope = Ast::Types::Scope(scope_ast);
        return scope;
    }

    let variable = Ast::VariableAST::new(&string);
    let variable = Ast::Types::Variable(variable);
    return variable;
}

fn function_call(tokens: &mut Vec<Token::TokenValue>) -> Vec<Ast::Types> {
    let mut vec_node: Vec<Ast::Types> = Vec::new();
    tokens.remove(0);

    loop {
        let token = tokens[0].token;
        if token == 40 || token == 44 {
            tokens.remove(0);
            continue;
        }

        if token == 41 {
            break;
        }

        let result = calculation(tokens);
        vec_node.push(result);
    }

    return vec_node;
}

fn calculation(tokens: &mut Vec<Token::TokenValue>) -> Ast::Types {
    let mut number_vector: Vec<Ast::Types> = Vec::new();
    let mut binary_vector: Vec<Ast::Types> = Vec::new();

    loop {
        let result = judge(tokens);

        match result {
            Ast::Types::Binary(_) => {
                binary_vector.push(result);
            }

            Ast::Types::Number(_) => {
                number_vector.push(result);
            }

            Ast::Types::Strings(_) => {
                number_vector.push(result);
            }

            Ast::Types::Variable(var) => {
                match var.index {
                    Some(_) => {
                        loop{
                            tokens.remove(0);
                            if tokens[0].token == 93 {
                                break;
                            }
                        }
                        number_vector.push(Ast::Types::Variable(var))
                    }
                    None => {number_vector.push(Ast::Types::Variable(var))}
                }
            }

            Ast::Types::Call(_) => match result {
                Ast::Types::Call(mut function) => {
                    function.argument = function_call(tokens);
                    let result = Ast::Types::Call(function);
                    number_vector.push(result);
                }

                _ => break,
            },
            Ast::Types::Scope(_) => {
                break;
            }

            Ast::Types::End(_) => {
                break;
            }
            Ast::Types::Comma(_) => {
                break;
            }

            Ast::Types::Parent(pra) => {
                if pra.parent == '(' {
                    tokens.remove(0);
                    continue;
                } else if pra.parent == ')' {
                    break;
                }
            }

            Ast::Types::Square(square) => {
                if square.square == '[' {
                    tokens.remove(0);
                    continue;
                }else if square.square == ']' {
                    break;
                }
            }

            _ => {
                break;
            }
        }

        tokens.remove(0);
    }

    if number_vector.len() < binary_vector.len() {
        let number = number_vector[0].clone();
        let binary = binary_vector[0].clone();
        let binary_sccond = binary_vector[1].clone();
        match binary {
            Ast::Types::Binary(mut bin) => {
                bin.node.push(number);
                bin.node.push(binary_sccond);
                return Ast::Types::Binary(bin);
            }

            _ => {}
        }
    }

    if number_vector.len() == 1 {
        let number = number_vector[0].clone();
        return number;
    }

    number_vector.reverse();
    binary_vector.reverse();

    let mut index = 0;
    let mut ast_temp = Ast::Types::Error(Ast::ErrorAST::new("ast_temp parsing error"));

    for binary in binary_vector {
        let mut number = number_vector[index].clone();

        if index > 0 {
            match number {
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

        _ => return Ast::Types::Error(Ast::ErrorAST::new("Binary parsing error")),
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
        }

        _ => {}
    }

    return result;
}

fn function(tokens: &mut Vec<Token::TokenValue>) -> Ast::Types {
    let string = &tokens[0].val;
    let token = tokens[1].token;
    let mut function_ast = Ast::FunctionAST::new(string);
    if token == 40 {
        tokens.remove(0);
        tokens.remove(0);
        loop {
            if tokens[0].token == -10 {
                let result = judge(tokens);
                function_ast.argument.push(result);
            }

            if tokens[0].token == 41 {
                break;
            }

            tokens.remove(0);
        }
    }

    let result = syntax(tokens);
    function_ast.node = result;
    return Ast::Types::Function(function_ast);
}

fn vector(tokens: &mut Vec<Token::TokenValue>) -> Vec<Ast::Types> {
    let mut vec: Vec<Ast::Types> = Vec::new();
    if tokens[0].token == 91 {
        loop {
            let result = calculation(tokens);
            vec.push(result);
            if tokens[0].token == 93 {
                tokens.remove(0);
                break;
            }
            tokens.remove(0);
        }
    }

    return vec;
}

fn syntax(tokens: &mut Vec<Token::TokenValue>) -> Vec<Ast::Types> {
    let mut node_vec = Vec::new();
    loop {
        if tokens.is_empty() {
            break;
        }

        let token = tokens[0].token;
        if token == 40 || token == 41 || token == 0 {
            tokens.remove(0);
            continue;
        }

        if token == 125 {
            break;
        }

        match scope(tokens) {
            Some(types) => {
                node_vec.push(types);
            }

            None => {
                continue;
            }
        }

        if tokens.is_empty() {
            break;
        }

        tokens.remove(0);
    }

    return node_vec;
}

fn scope(tokens: &mut Vec<Token::TokenValue>) -> Option<Ast::Types> {
    let mut result = judge(tokens);
    match result {
        Ast::Types::Call(mut function) => {
            function.argument = function_call(tokens);
            result = Ast::Types::Call(function);
            return Some(result);
        }

        Ast::Types::Function(_) => {
            return Some(result);
        }

        Ast::Types::Number(_) => {
            result = calculation(tokens);
            return Some(result);
        }

        Ast::Types::Variable(mut var) => {
            let result_var = variable(tokens);
            let continue_tokne = tokens[0].token;

            if continue_tokne == 59 {
                var.node.push(result_var);
                result = Ast::Types::Variable(var);
                return Some(result);
            }

            let result_cal = calculation(tokens);
            var.node.push(result_cal);
            result = Ast::Types::Variable(var);
            return Some(result);
        }

        Ast::Types::If(mut ifs) => {
            let result = syntax(tokens);
            ifs.node = result;
            return Some(Ast::Types::If(ifs));
        }

        Ast::Types::For(mut fors) => {
            let result = syntax(tokens);
            fors.node = result;
            return Some(Ast::Types::For(fors));
        }

        Ast::Types::Vector(_) => {
            return Some(result);
        }

        Ast::Types::Retrun(_) => {
            return Some(result);
        }

        Ast::Types::End(_) => {
            tokens.remove(0);
        }

        _ => {
            if !tokens.is_empty() {
                tokens.remove(0);
            }
        }
    }
    return None;
}
