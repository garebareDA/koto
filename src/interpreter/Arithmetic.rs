use super::super::ast::Ast;

pub fn common(bin: Ast::BinaryAST) {
    let op = bin.op;
    let node = bin.node[0].clone();
    let mut next_node = bin.node[1].clone();

    if op == '%' {
        let (numbers, types) =  match_type(node.clone(), next_node.clone());
        let result = modulo(numbers[0], numbers[1]);
        let mut ast = Ast::NumberAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = Ast::Types::Number(ast);
        }
    }

    if op == '*' {
        let (numbers, types) =  match_type(node.clone(), next_node.clone());
        let result = multiplication(numbers[0], numbers[1]);
        let mut ast = Ast::NumberAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = Ast::Types::Number(ast);
        }
    }

    if op == '/' {
        let (numbers, types) =  match_type(node.clone(), next_node.clone());
        let result = division(numbers[0], numbers[1]);
        let mut ast = Ast::NumberAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = Ast::Types::Number(ast);
        }
    }

    next_node = calculattions(next_node, 1);

    if op == '-' {
        let (numbers, types) =  match_type(node.clone(), next_node.clone());
        let result = minus(numbers[0], numbers[1]);
        let mut ast = Ast::NumberAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = Ast::Types::Number(ast);
        }
    }

    if op == '+' {
        let (numbers, types) =  match_type(node.clone(), next_node.clone());
        let result = plus(numbers[0], numbers[1]);
        let mut ast = Ast::NumberAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = Ast::Types::Number(ast);
        }
    }

    next_node = calculattions(next_node, 2);
    println!("{:?}", next_node);
}

fn match_type(node:Ast::Types, next_node:Ast::Types) -> (Vec<i64>, Vec<Ast::Types>) {
    let mut numbers:Vec<i64> = Vec::new();
    let mut types:Vec<Ast::Types> = Vec::new();

    match node {
        Ast::Types::Number(num) => {
            numbers.push(num.val);
        }
        _ => {}
    }

    match next_node {
        Ast::Types::Number(num) => {
            numbers.push(num.val);
            if !num.node.is_empty() {
                types.push(num.node[0].clone());
            }
        }
        _ => {}
    }

    return (numbers, types);
}

fn calculattions(numbers: Ast::Types, select_binary: i64) -> Ast::Types{
    match numbers.clone() {
        Ast::Types::Number(num) => {
            if num.node.is_empty() {
                return numbers;
            }
            let number_a = num.val;
            let node_first = num.node[0].clone();

            match node_first {
                Ast::Types::Binary(binary) => {
                    let bin = binary.op;
                    let node = binary.node[0].clone();

                    match node.clone() {
                        Ast::Types::Number(num) => {
                            let number_b = num.val;
                            if bin == '%' && select_binary == 1 {
                                let result = modulo(number_a, number_b);
                                if num.node.is_empty() {
                                    let ast = Ast::NumberAST::new(result);
                                    return Ast::Types::Number(ast);
                                }
                                let number_type = calculattions_common(result, num);
                                let number_result = calculattions(number_type, select_binary);
                                return number_result
                            }

                            if bin == '*' && select_binary == 1 {
                                let result = multiplication(number_a, number_b);
                                if num.node.is_empty() {
                                    let ast = Ast::NumberAST::new(result);
                                    return Ast::Types::Number(ast);
                                }
                                let number_type = calculattions_common(result, num);
                                let number_result = calculattions(number_type, select_binary);
                                return number_result
                            }

                            if bin == '/' && select_binary == 1 {
                                let result = division(number_a, number_b);
                                if num.node.is_empty() {
                                    let ast = Ast::NumberAST::new(result);
                                    return Ast::Types::Number(ast);
                                }
                                let number_type = calculattions_common(result, num);
                                let number_result = calculattions(number_type, select_binary);
                                return number_result
                            }

                            if bin == '-' && select_binary == 2 {
                                let result = minus(number_a, number_b);
                                if num.node.is_empty() {
                                    let ast = Ast::NumberAST::new(result);
                                    return Ast::Types::Number(ast);
                                }
                                let number_type = calculattions_common(result, num);
                                let number_result = calculattions(number_type, select_binary);
                                return number_result
                            }

                            if bin == '+' && select_binary == 2 {
                                let result = plus(number_a, number_b);
                                if num.node.is_empty() {
                                    let ast = Ast::NumberAST::new(result);
                                    return Ast::Types::Number(ast);
                                }
                                let number_type = calculattions_common(result, num);
                                let number_result = calculattions(number_type, select_binary);
                                return number_result
                            }

                            if !num.node.is_empty() {
                                let result = calculattions(node.clone(), select_binary);

                                let mut binarys = Ast::BinaryAST::new(bin);
                                binarys.node.push(result);

                                let mut numbers = Ast::NumberAST::new(number_a);
                                numbers.node.push(Ast::Types::Binary(binarys));

                                return Ast::Types::Number(numbers);
                            }

                            return numbers;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
    return numbers;
}

fn calculattions_common(num:i64, ast:Ast::NumberAST) -> Ast::Types {
    let mut number_ast = Ast::NumberAST::new(num);
    let node_number = ast.node[0].clone();
    number_ast.node.push(node_number);
    return Ast::Types::Number(number_ast);
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