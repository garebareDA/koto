use super::super::lexer::token;
use super::asts;

pub struct Parsing {
    pub tokens: Vec<token::TokenValue>,
}

impl Parsing {
    pub fn new(tokens: &Vec<token::TokenValue>) -> Parsing {
        let tokens = tokens.clone();
        Parsing { tokens: tokens }
    }

    pub fn parsing(&mut self) -> asts::ExprAST {
        let mut root = asts::ExprAST::new();
        let result = self.syntax();
        root.node = result;
        return root;
    }

    fn judge(&mut self) -> asts::Types {
        let mut token = self.tokens[0].token;
        let mut string = self.tokens[0].val.clone();
        let token_constant = token::Token::new();

        if token == token_constant._comment {
            self.tokens.remove(0);
            token = self.tokens[0].token;
            string = self.tokens[0].val.clone();
        }
        if token == token_constant._if {
            self.tokens.remove(0);
            let result = self.calculation();
            let mut result_vec: Vec<asts::Types> = Vec::new();
            result_vec.push(result);
            let if_ast = asts::IfAST::new(result_vec);
            return asts::Types::If(if_ast);
        }
        if token == token_constant._for {
            self.tokens.remove(0);
            let var = self.scope();
            self.tokens.remove(0);
            let result = self.calculation();
            self.tokens.remove(0);
            let loop_op = self.judge();
            match var {
                Some(var) => {
                    let for_ast = asts::ForAST::new(var, result, loop_op);
                    let for_types = asts::Types::For(for_ast);
                    return for_types;
                }
                None => {
                    return asts::Types::Error(asts::ErrorAST::new("for parsing error"));
                }
            }
        }
        if token == token_constant._fun {
            self.tokens.remove(0);
            return self.function();
        }
        if token == token_constant._print {
            let print = asts::CallAST::new("print");
            let call = asts::Types::Call(print);
            return call;
        }
        if token == token_constant._string {
            let string = asts::StringAST::new(&string);
            let strings = asts::Types::Strings(string);
            return strings;
        }
        if token == token_constant._number {
            let num = string.parse().unwrap();
            let num = asts::NumberAST::new(num);
            let number = asts::Types::Number(num);
            return number;
        }
        if token == token_constant._identifier {
            if self.tokens[1].token == 40 {
                let call = asts::CallAST::new(&string);
                let call = asts::Types::Call(call);
                return call;
            }
            if self.tokens[1].token == 91 {
                //tokensに再代入しているので思いつき次第改善
                let before = self.tokens.clone();
                self.tokens.remove(0);
                let index = self.calculation();
                let mut variable = asts::VariableAST::new(&string);
                variable.index = Some(vec![index]);
                let variable = asts::Types::Variable(variable);
                self.tokens = before;
                return variable;
            }

            if self.tokens[2].token == 43 || self.tokens[2].token == 45 {
                return self.reassignment();
            }

            let variable = asts::VariableAST::new(&string);
            let variable = asts::Types::Variable(variable);
            return variable;
        }
        if token == token_constant._let {
            self.tokens.remove(0);
            let string = &self.tokens[0].val;
            let variable = asts::VariableAST::new(string);
            let variable = asts::Types::Variable(variable);
            self.tokens.remove(0);
            self.tokens.remove(0);
            return variable;
        }
        if token == token_constant._bool {
            if string == "true" {
                let bools = asts::BooleanAST::new(true);
                let bools = asts::Types::Boolean(bools);
                return bools;
            } else if string == "false" {
                let bools = asts::BooleanAST::new(false);
                let bools = asts::Types::Boolean(bools);
                return bools;
            }
        }
        if token == token_constant._return {
            self.tokens.remove(0);
            let mut retrun_ast = asts::RetrunAST::new();
            if self.tokens[0].token == 59 {
                return asts::Types::Retrun(retrun_ast);
            }
            let result = self.judge();
            retrun_ast.node.push(result);
            return asts::Types::Retrun(retrun_ast);
        }
        if token == token_constant._vec {
            self.tokens.remove(0);
            let result = self.vector();
            let mut vector_ast = asts::VectorAST::new();
            vector_ast.node = result;
            return asts::Types::Vector(vector_ast);
        }
        if token == 40 || token == 41 {
            let parent = asts::ParenthesesAST::new(string.chars().nth(0).unwrap());
            let parent = asts::Types::Parent(parent);
            return parent;
        }
        if token == 44 {
            let comma = asts::CommaAST::new(',');
            let comma = asts::Types::Comma(comma);
            return comma;
        }
        if token == 43 || token == 45 || token == 47 || token == 37 || token == 42 {
            let bin = asts::BinaryAST::new(string.parse().unwrap());
            let binary = asts::Types::Binary(bin);
            return binary;
        }
        if token == 60 || token == 62 || token == 124 || token == 61 || token == 33 || token == 38 {
            let mut bin = asts::BinaryAST::new(string.parse().unwrap());
            let token = self.tokens[1].token;
            if token == 61 || token == 38 || token == 124 {
                self.tokens.remove(0);
                let string = self.tokens[0].val.clone();
                let in_binary = asts::BinaryAST::new(string.parse().unwrap());
                let in_binary = asts::Types::Binary(in_binary);
                bin.node.push(in_binary);
            }
            let binary = asts::Types::Binary(bin);
            return binary;
        }
        if token == 59 {
            let end = asts::EndAST::new();
            let end = asts::Types::End(end);
            return end;
        }
        if token == 91 || token == 93 {
            let squar = asts::SquareAST::new(string.parse().unwrap());
            let squar = asts::Types::Square(squar);
            return squar;
        }
        if token == 123 || token == 125 {
            let scope_ast = asts::ScopeAST::new(string.parse().unwrap());
            let scope = asts::Types::Scope(scope_ast);
            return scope;
        }

        let variable = asts::VariableAST::new(&string);
        let variable = asts::Types::Variable(variable);
        return variable;
    }

    fn function_call(&mut self) -> Vec<asts::Types> {
        let mut vec_node: Vec<asts::Types> = Vec::new();
        self.tokens.remove(0);
        loop {
            let token = self.tokens[0].token;
            if token == 40 || token == 44 {
                self.tokens.remove(0);
                continue;
            }
            if token == 41 {
                break;
            }
            let result = self.calculation();
            vec_node.push(result);
        }
        return vec_node;
    }

    fn calculation(&mut self) -> asts::Types {
        let mut number_vector: Vec<asts::Types> = Vec::new();
        let mut binary_vector: Vec<asts::Types> = Vec::new();
        loop {
            let result = self.judge();
            match result {
                asts::Types::Binary(_) => {
                    binary_vector.push(result);
                }
                asts::Types::Number(_) => {
                    number_vector.push(result);
                }
                asts::Types::Strings(_) => {
                    number_vector.push(result);
                }

                asts::Types::Variable(var) => match var.index {
                    Some(_) => {
                        loop {
                            self.tokens.remove(0);
                            if self.tokens[0].token == 93 {
                                break;
                            }
                        }
                        number_vector.push(asts::Types::Variable(var))
                    }
                    None => number_vector.push(asts::Types::Variable(var)),
                },
                asts::Types::Call(_) => match result {
                    asts::Types::Call(mut function) => {
                        function.argument = self.function_call();
                        let result = asts::Types::Call(function);
                        number_vector.push(result);
                    }
                    _ => break,
                },
                asts::Types::Scope(_) => {
                    break;
                }
                asts::Types::End(_) => {
                    break;
                }
                asts::Types::Comma(_) => {
                    break;
                }
                asts::Types::Parent(pra) => {
                    if pra.parent == '(' {
                        self.tokens.remove(0);
                        continue;
                    } else if pra.parent == ')' {
                        break;
                    }
                }
                asts::Types::Square(square) => {
                    if square.square == '[' {
                        self.tokens.remove(0);
                        continue;
                    } else if square.square == ']' {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
            self.tokens.remove(0);
        }

        if number_vector.len() < binary_vector.len() {
            let number = number_vector[0].clone();
            let binary = binary_vector[0].clone();
            let binary_sccond = binary_vector[1].clone();
            match binary {
                asts::Types::Binary(mut bin) => {
                    bin.node.push(number);
                    bin.node.push(binary_sccond);
                    return asts::Types::Binary(bin);
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
        let mut ast_temp = asts::Types::Error(asts::ErrorAST::new("ast_temp parsing error"));
        for binary in binary_vector {
            let mut number = number_vector[index].clone();
            if index > 0 {
                match number {
                    asts::Types::Number(mut numbers) => {
                        numbers.node.push(ast_temp.clone());
                        number = asts::Types::Number(numbers);
                    }

                    asts::Types::Strings(mut strings) => {
                        strings.node.push(ast_temp.clone());
                        number = asts::Types::Strings(strings);
                    }
                    _ => {}
                }
            }

            match binary {
                asts::Types::Binary(mut binary) => {
                    binary.node.push(number.clone());
                    ast_temp = asts::Types::Binary(binary);
                }
                _ => {}
            }
            index += 1;
        }

        match ast_temp {
            asts::Types::Binary(mut binary) => {
                binary.node.push(number_vector[index].clone());
                binary.node.reverse();
                let ast_binary = asts::Types::Binary(binary);
                return ast_binary;
            }
            _ => return asts::Types::Error(asts::ErrorAST::new("Binary parsing error")),
        }
    }

    fn variable(&mut self) -> asts::Types {
        let token = self.tokens[0].token;
        if token == 40 || token == 41 {
            self.tokens.remove(0);
        }
        let mut result = self.judge();
        match result {
            asts::Types::Number(_) => {
                result = self.calculation();
            }
            _ => {}
        }
        return result;
    }

    fn reassignment(&mut self) -> asts::Types {
        let var_val = asts::VariableAST::new(&self.tokens[0].val);
        self.tokens.remove(0);
        let first_bin = self.judge();
        self.tokens.remove(0);
        let second_bin = self.judge();
        self.tokens.remove(0);

        match first_bin {
            asts::Types::Binary(mut bin) => {
                bin.node.push(asts::Types::Variable(var_val));
                bin.node.push(second_bin);
                return asts::Types::Binary(bin);
            }

            _ =>{return asts::Types::Error(asts::ErrorAST::new("Reassignment operater error"));}
        }
    }

    fn function(&mut self) -> asts::Types {
        let string = &self.tokens[0].val;
        let token = self.tokens[1].token;
        let mut function_ast = asts::FunctionAST::new(string);
        if token == 40 {
            self.tokens.remove(0);
            self.tokens.remove(0);
            loop {
                let token_judge = self.tokens[0].token;
                if token_judge == -10 {
                    let result = self.judge();
                    function_ast.argument.push(result);
                }
                if token_judge == 41 {
                    break;
                }
                self.tokens.remove(0);
            }
        }
        let result = self.syntax();
        function_ast.node = result;
        return asts::Types::Function(function_ast);
    }

    fn vector(&mut self) -> Vec<asts::Types> {
        let mut vec: Vec<asts::Types> = Vec::new();
        if self.tokens[0].token == 91 {
            loop {
                let result = self.calculation();
                vec.push(result);
                if self.tokens[0].token == 93 {
                    self.tokens.remove(0);
                    break;
                }
                self.tokens.remove(0);
            }
        }
        return vec;
    }

    fn syntax(&mut self) -> Vec<asts::Types> {
        let mut node_vec = Vec::new();
        loop {
            if self.tokens.is_empty() {
                break;
            }
            let token = self.tokens[0].token;
            if token == 40 || token == 41 || token == 0 {
                self.tokens.remove(0);
                continue;
            }
            if token == 125 {
                break;
            }
            match self.scope() {
                Some(types) => {
                    node_vec.push(types);
                }
                None => {
                    continue;
                }
            }
            if self.tokens.is_empty() {
                break;
            }
            self.tokens.remove(0);
        }
        return node_vec;
    }

    fn scope(&mut self) -> Option<asts::Types> {
        let mut result = self.judge();
        match result {
            asts::Types::Call(mut function) => {
                function.argument = self.function_call();
                result = asts::Types::Call(function);
                return Some(result);
            }
            asts::Types::Function(_) => {
                return Some(result);
            }
            asts::Types::Number(_) => {
                result = self.calculation();
                return Some(result);
            }
            asts::Types::Variable(mut var) => {
                let result_var = self.variable();
                let continue_tokne = self.tokens[0].token;
                if continue_tokne == 59 {
                    var.node.push(result_var);
                    result = asts::Types::Variable(var);
                    return Some(result);
                }
                let result_cal = self.calculation();
                var.node.push(result_cal);
                result = asts::Types::Variable(var);
                return Some(result);
            }
            asts::Types::If(mut ifs) => {
                let result = self.syntax();
                ifs.node = result;
                return Some(asts::Types::If(ifs));
            }
            asts::Types::For(mut fors) => {
                let result = self.syntax();
                fors.node = result;
                return Some(asts::Types::For(fors));
            }
            asts::Types::Binary(_) => {
                return Some(result);
            }
            asts::Types::Vector(_) => {
                return Some(result);
            }
            asts::Types::Retrun(_) => {
                return Some(result);
            }
            asts::Types::End(_) => {
                self.tokens.remove(0);
            }
            _ => {
                if !self.tokens.is_empty() {
                    self.tokens.remove(0);
                }
            }
        }
        return None;
    }
}
