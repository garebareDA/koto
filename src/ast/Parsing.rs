use super::super::lexer::Token;
use super::Ast;

//構文解析
pub fn parsing(tokens: &mut Vec<Token::TokenValue>) -> Ast::ExprAST {
    let mut root = Ast::ExprAST::new();

    loop {
        if tokens.is_empty() {
            break;
        }

        let token = tokens[0].token;

        if token == 40 || token == 41 {
            tokens.remove(0);
            continue;
        }

        match scope(tokens) {
            Some(types) => {
                root.node.push(types);
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

    return root;
}

fn judge(tokens: &mut Vec<Token::TokenValue>) -> Ast::Types {
    let token = tokens[0].token;
    let string = tokens[0].val.clone();

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
            Some(var) =>{
                let for_ast = Ast::ForAST::new(var, result, loop_op);
                let for_types = Ast::Types::For(for_ast);
                return for_types;
            }

            None =>{return Ast::Types::Error(Ast::ErrorAST::new("for parsing error"));}
        }
    }

    if token == -5 {
        //TODO 関数の処理
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
        if tokens[1].token == 40{
            //TODO 関数の処理
        }
        let variable = Ast::VariableAST::new(&string);
        let variable = Ast::Types::Variabel(variable);
        return variable;
    }

    if token == -11 {
        tokens.remove(0);
        let string = tokens[0].val.clone();
        let variable = Ast::VariableAST::new(&string);
        let variable = Ast::Types::Variabel(variable);
        tokens.remove(0);
        tokens.remove(0);
        return variable;
    }

    if token == -12 {
        if string == "true" {
            let bools = Ast::BooleanAST::new(true);
            let bools = Ast::Types::Boolean(bools);
            return bools;
        }else if string == "false" {
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

    if token == 123 || token == 125 {
        let scope_ast = Ast::ScopeAST::new(string.chars().nth(0).unwrap());
        let scope = Ast::Types::Scope(scope_ast);
        return scope;
    }

    let variable = Ast::VariableAST::new(&string);
    let variable = Ast::Types::Variabel(variable);
    return variable;
}

fn function_call(tokens: &mut Vec<Token::TokenValue>) -> Ast::Types {
    let string = Ast::StringAST::new("");
    let mut result = Ast::Types::Strings(string);

    loop {
        let token = tokens[0].token;

        if token == 40 {
            tokens.remove(0);
            continue;
        }

        if token == 41 {
            break;
        }

        result = judge(tokens);
        tokens.remove(0);
    }

    return result;
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

            Ast::Types::Variabel(_) => {
                number_vector.push(result);
            }

            Ast::Types::Scope(_) => {
                break;
            }

            Ast::Types::End(_) => {
                break;
            }

            _ => {}
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

            _ => {Ast::Types::Error(Ast::ErrorAST::new("binary parsing error"));}
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

fn if_syntax(tokens: &mut Vec<Token::TokenValue>, mut if_ast: Ast::IfAST) -> Ast::Types {
    loop {
        if tokens.is_empty() {
            break;
        }

        let token = tokens[0].token;
        if token == 40 || token == 41 {
            tokens.remove(0);
            continue;
        }

        if token == 125{
            tokens.remove(0);
            break;
        }

        match scope(tokens) {
            Some(types) => {
                if_ast.node.push(types);
            }

            None => {
                continue;
            }
        }

        tokens.remove(0);
    }

    return Ast::Types::If(if_ast);
}

fn for_syntax(tokens: &mut Vec<Token::TokenValue>, mut for_ast: Ast::ForAST) -> Ast::Types {
    loop {
        if tokens.is_empty() {
            break;
        }

        let token = tokens[0].token;
        if token == 40 || token == 41 {
            tokens.remove(0);
            continue;
        }

        if token == 125{
            break;
        }

        match scope(tokens) {
            Some(types) => {
                for_ast.node.push(types);
            }

            None => {
                continue;
            }
        }

        tokens.remove(0);
    }

    return Ast::Types::For(for_ast);
}

fn scope(tokens: &mut Vec<Token::TokenValue>) -> Option<Ast::Types> {
    let mut result = judge(tokens);
    match result {
        Ast::Types::Call(mut function) => {
            let result_call = function_call(tokens);
            function.node.push(result_call);
            result = Ast::Types::Call(function);
            return Some(result);
        }

        Ast::Types::Number(_) => {
            result = calculation(tokens);
            return Some(result);
        }

        Ast::Types::Variabel(mut var) => {
            let result_var = variable(tokens);
            let continue_tokne = tokens[0].token;

            if continue_tokne == 59 {
                var.node.push(result_var);
                result = Ast::Types::Variabel(var);
                return Some(result);
            }


            let result_cal = calculation(tokens);
            var.node.push(result_cal);
            result = Ast::Types::Variabel(var);
            return Some(result);
        }

        Ast::Types::If(ifs) => {
            let result = if_syntax(tokens, ifs);
            return Some(result);
        }

        Ast::Types::For(fors) => {
            let result = for_syntax(tokens, fors);
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