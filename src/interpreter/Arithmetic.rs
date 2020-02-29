use super::super::ast::Ast;

pub fn common(bin: Ast::BinaryAST) {
    let op = bin.op;
    let node = bin.node[0].clone();
    let mut next_node = bin.node[1].clone();

    if op == '%' {
        let mut numbers:Vec<i64> = Vec::new();
        let mut types:Vec<Ast::Types> = Vec::new();

        match node.clone() {
            Ast::Types::Number(num) => {
                numbers.push(num.val);
            }
            _ => {}
        }

        match next_node {
            Ast::Types::Number(num) => {
                numbers.push(num.val);
                types.push(num.node[0].clone());
            }
            _ => {}
        }

        let result = modulo(numbers[0], numbers[1]);
        let mut ast = Ast::NumberAST::new(result);
        ast.node.push(types[0].clone());
        next_node = Ast::Types::Number(ast);
    }


    if op == '*' {
        let mut numbers:Vec<i64> = Vec::new();
        let mut types:Vec<Ast::Types> = Vec::new();

        match node.clone() {
            Ast::Types::Number(num) => {
                numbers.push(num.val);
            }
            _ => {}
        }

        match next_node {
            Ast::Types::Number(num) => {
                numbers.push(num.val);
                types.push(num.node[0].clone());
            }
            _ => {}
        }

        let result = multiplication(numbers[0], numbers[1]);
        let mut ast = Ast::NumberAST::new(result);
        ast.node.push(types[0].clone());
        next_node = Ast::Types::Number(ast);
    }

    if op == '/' {
        let mut numbers:Vec<i64> = Vec::new();
        let mut types:Vec<Ast::Types> = Vec::new();

        match node.clone() {
            Ast::Types::Number(num) => {
                numbers.push(num.val);
            }
            _ => {}
        }

        match next_node {
            Ast::Types::Number(num) => {
                numbers.push(num.val);
                types.push(num.node[0].clone());
            }
            _ => {}
        }

        let result = division(numbers[0], numbers[1]);
        let mut ast = Ast::NumberAST::new(result);
        ast.node.push(types[0].clone());
        next_node = Ast::Types::Number(ast);
    }

    if op == '-' {
        let mut numbers:Vec<i64> = Vec::new();
        let mut types:Vec<Ast::Types> = Vec::new();

        match node.clone() {
            Ast::Types::Number(num) => {
                numbers.push(num.val);
            }
            _ => {}
        }

        match next_node {
            Ast::Types::Number(num) => {
                numbers.push(num.val);
                types.push(num.node[0].clone());
            }
            _ => {}
        }

        let result = minus(numbers[0], numbers[1]);
        let mut ast = Ast::NumberAST::new(result);
        ast.node.push(types[0].clone());
        next_node = Ast::Types::Number(ast);
    }

    if op == '+' {
        let mut numbers:Vec<i64> = Vec::new();
        let mut types:Vec<Ast::Types> = Vec::new();

        match node.clone() {
            Ast::Types::Number(num) => {
                numbers.push(num.val);
            }
            _ => {}
        }

        match next_node {
            Ast::Types::Number(num) => {
                numbers.push(num.val);
                types.push(num.node[0].clone());
            }
            _ => {}
        }

        let result = plus(numbers[0], numbers[1]);
        let mut ast = Ast::NumberAST::new(result);
        ast.node.push(types[0].clone());
        next_node = Ast::Types::Number(ast);
    }
}

fn calculation_modulo(numbers: Ast::Types){
    match numbers {
        Ast::Types::Number(num) => {
            
        }
        _ => {}
    }
}



fn modulo(a: i64, b: i64) -> i64 {
    a % b
}

fn multiplication(a: i64, b: i64) -> i64 {
    a * b
}

fn division(a: i64, b: i64) -> i64 {
    a / b
}

fn minus(a: i64, b: i64) -> i64 {
    a - b
}

fn plus(a: i64, b: i64) -> i64 {
    a + b
}