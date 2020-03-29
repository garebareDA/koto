use super::super::lexer::Token;
use super::Ast;

pub struct Parsing {
    pub tokens: Vec<Token::TokenValue>,
}

impl Parsing {
    pub fn new(tokens: &Vec<Token::TokenValue>) -> Parsing {
        let tokens = tokens.clone();
        Parsing { tokens: tokens }
    }

    pub fn parsing(&mut self) -> Ast::ExprAST {
        let mut root = Ast::ExprAST::new();
        let result = self.syntax();
        root.node = result;
        return root;
    }

    fn judge(&mut self) -> Ast::Types {
        let mut token = self.tokens[0].token;
        let mut string = self.tokens[0].val.clone();
        let token_constant = Token::Token::new();

        if token == token_constant._comment {
            self.tokens.remove(0);
            token = self.tokens[0].token;
            string = self.tokens[0].val.clone();
        }
        if token == token_constant._if {
            self.tokens.remove(0);
            let result = self.calculation();
            let mut result_vec: Vec<Ast::Types> = Vec::new();
            result_vec.push(result);
            let if_ast = Ast::IfAST::new(result_vec);
            return Ast::Types::If(if_ast);
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
                    let for_ast = Ast::ForAST::new(var, result, loop_op);
                    let for_types = Ast::Types::For(for_ast);
                    return for_types;
                }
                None => {
                    return Ast::Types::Error(Ast::ErrorAST::new("for parsing error"));
                }
            }
        }
        if token == token_constant._fun {
            self.tokens.remove(0);
            return self.function();
        }
        if token == token_constant._print {
            let print = Ast::CallAST::new("print");
            let call = Ast::Types::Call(print);
            return call;
        }
        if token == token_constant._string {
            let string = Ast::StringAST::new(&string);
            let strings = Ast::Types::Strings(string);
            return strings;
        }
        if token == token_constant._number {
            let num = string.parse().unwrap();
            let num = Ast::NumberAST::new(num);
            let number = Ast::Types::Number(num);
            return number;
        }
        if token == token_constant._identifier {
            if self.tokens[1].token == 40 {
                let call = Ast::CallAST::new(&string);
                let call = Ast::Types::Call(call);
                return call;
            }
            if self.tokens[1].token == 91 {
                //tokensに再代入しているので思いつき次第改善
                let before = self.tokens.clone();
                self.tokens.remove(0);
                let index = self.calculation();
                let mut variable = Ast::VariableAST::new(&string);
                variable.index = Some(vec![index]);
                let variable = Ast::Types::Variable(variable);
                self.tokens = before;
                return variable;
            }

            if self.tokens[2].token == 43 || self.tokens[2].token == 45 {
                return self.reassignment();
            }

            let variable = Ast::VariableAST::new(&string);
            let variable = Ast::Types::Variable(variable);
            return variable;
        }
        if token == token_constant._let {
            self.tokens.remove(0);
            let string = &self.tokens[0].val;
            let variable = Ast::VariableAST::new(string);
            let variable = Ast::Types::Variable(variable);
            self.tokens.remove(0);
            self.tokens.remove(0);
            return variable;
        }
        if token == token_constant._bool {
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
        if token == token_constant._return {
            self.tokens.remove(0);
            let mut retrun_ast = Ast::RetrunAST::new();
            if self.tokens[0].token == 59 {
                return Ast::Types::Retrun(retrun_ast);
            }
            let result = self.judge();
            retrun_ast.node.push(result);
            return Ast::Types::Retrun(retrun_ast);
        }
        if token == token_constant._vec {
            self.tokens.remove(0);
            let result = self.vector();
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
            let token = self.tokens[1].token;
            if token == 61 || token == 38 || token == 124 {
                self.tokens.remove(0);
                let string = self.tokens[0].val.clone();
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

    fn function_call(&mut self) -> Vec<Ast::Types> {
        let mut vec_node: Vec<Ast::Types> = Vec::new();
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

    fn calculation(&mut self) -> Ast::Types {
        let mut number_vector: Vec<Ast::Types> = Vec::new();
        let mut binary_vector: Vec<Ast::Types> = Vec::new();
        loop {
            let result = self.judge();
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

                Ast::Types::Variable(var) => match var.index {
                    Some(_) => {
                        loop {
                            self.tokens.remove(0);
                            if self.tokens[0].token == 93 {
                                break;
                            }
                        }
                        number_vector.push(Ast::Types::Variable(var))
                    }
                    None => number_vector.push(Ast::Types::Variable(var)),
                },
                Ast::Types::Call(_) => match result {
                    Ast::Types::Call(mut function) => {
                        function.argument = self.function_call();
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
                        self.tokens.remove(0);
                        continue;
                    } else if pra.parent == ')' {
                        break;
                    }
                }
                Ast::Types::Square(square) => {
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

    fn variable(&mut self) -> Ast::Types {
        let token = self.tokens[0].token;
        if token == 40 || token == 41 {
            self.tokens.remove(0);
        }
        let mut result = self.judge();
        match result {
            Ast::Types::Number(_) => {
                result = self.calculation();
            }
            _ => {}
        }
        return result;
    }

    fn reassignment(&mut self) -> Ast::Types {
        let var_val = Ast::VariableAST::new(&self.tokens[0].val);
        self.tokens.remove(0);
        let first_bin = self.judge();
        self.tokens.remove(0);
        let second_bin = self.judge();
        self.tokens.remove(0);

        match first_bin {
            Ast::Types::Binary(mut bin) => {
                bin.node.push(Ast::Types::Variable(var_val));
                bin.node.push(second_bin);
                println!("{:?}", bin);
                return Ast::Types::Binary(bin);
            }

            _ =>{return Ast::Types::Error(Ast::ErrorAST::new("Reassignment operater error"));}
        }
    }

    fn function(&mut self) -> Ast::Types {
        let string = &self.tokens[0].val;
        let token = self.tokens[1].token;
        let mut function_ast = Ast::FunctionAST::new(string);
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
        return Ast::Types::Function(function_ast);
    }

    fn vector(&mut self) -> Vec<Ast::Types> {
        let mut vec: Vec<Ast::Types> = Vec::new();
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

    fn syntax(&mut self) -> Vec<Ast::Types> {
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

    fn scope(&mut self) -> Option<Ast::Types> {
        let mut result = self.judge();
        match result {
            Ast::Types::Call(mut function) => {
                function.argument = self.function_call();
                result = Ast::Types::Call(function);
                return Some(result);
            }
            Ast::Types::Function(_) => {
                return Some(result);
            }
            Ast::Types::Number(_) => {
                result = self.calculation();
                return Some(result);
            }
            Ast::Types::Variable(mut var) => {
                let result_var = self.variable();
                let continue_tokne = self.tokens[0].token;
                if continue_tokne == 59 {
                    var.node.push(result_var);
                    result = Ast::Types::Variable(var);
                    return Some(result);
                }
                let result_cal = self.calculation();
                var.node.push(result_cal);
                result = Ast::Types::Variable(var);
                return Some(result);
            }
            Ast::Types::If(mut ifs) => {
                let result = self.syntax();
                ifs.node = result;
                return Some(Ast::Types::If(ifs));
            }
            Ast::Types::For(mut fors) => {
                let result = self.syntax();
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
