use super::super::ast::Ast;
//TODO比較演算子の追加

pub fn common(bin: Ast::BinaryAST) -> Ast::Types {
    let op = bin.op;
    let node = bin.node[0].clone();
    let mut next_node = bin.node[1].clone();

    if op == '%' {
        let (numbers, types) = match_type(node.clone(), next_node.clone());
        let result = modulo(numbers[0], numbers[1]);
        let mut ast = Ast::NumberAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = Ast::Types::Number(ast);
        }
    }

    if op == '*' {
        let (numbers, types) = match_type(node.clone(), next_node.clone());
        let result = multiplication(numbers[0], numbers[1]);
        let mut ast = Ast::NumberAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = Ast::Types::Number(ast);
        }
    }

    if op == '/' {
        let (numbers, types) = match_type(node.clone(), next_node.clone());
        let result = division(numbers[0], numbers[1]);
        let mut ast = Ast::NumberAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = Ast::Types::Number(ast);
        }
    }

    next_node = calculattions(next_node, 1);

    if op == '-' {
        let (numbers, types) = match_type(node.clone(), next_node.clone());
        let result = minus(numbers[0], numbers[1]);
        let mut ast = Ast::NumberAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = Ast::Types::Number(ast);
        }
    }

    if op == '+' {
        let (numbers, types) = match_type(node.clone(), next_node.clone());
        let result = plus(numbers[0], numbers[1]);
        let mut ast = Ast::NumberAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = Ast::Types::Number(ast);
        }
    }

    next_node = calculattions(next_node, 2);

    next_node = calculattions(next_node, 3);
    next_node = calculattions(next_node, 4);
    next_node = calculattions(next_node, 5);
    next_node = calculattions(next_node, 6);
    println!("{:?}", next_node);
    return next_node;
}

fn calculattions(numbers: Ast::Types, select_binary: i64) -> Ast::Types {
    let mut number_a = 0;
    let mut bool_a = false;
    let mut bin = ' ';

    let mut node_first = Ast::Types::Error(Ast::ErrorAST::new("variable error"));
    let mut node_seccond = Ast::Types::Error(Ast::ErrorAST::new("variable error"));
    let mut comparison_node = Ast::Types::Error(Ast::ErrorAST::new("variable error"));

    let priority_first = 1;
    let priority_seccond = 2;
    let priority_therd = 3;
    let priority_forth = 4;
    let priority_fifth = 5;
    let priority_sixs = 6;

    match numbers.clone() {
        Ast::Types::Number(num) => {
            if num.node.is_empty() {
                return numbers;
            }
            number_a = num.val;
            node_first = num.node[0].clone();
        }

        Ast::Types::Boolean(bools) => {
            if bools.node.is_empty() {
                return numbers;
            }
            bool_a = bools.boolean;
            node_first = bools.node[0].clone();
        }
        _ => {}
    }

    match node_first {
        Ast::Types::Binary(binary) => {
            bin = binary.op;
            let len = binary.node.len();

            if len == 1 {
                node_seccond = binary.node[0].clone();
            } else if len == 2 {
                comparison_node = binary.node[0].clone();
                node_seccond = binary.node[1].clone();
            } else {
                panic!("binary erorr");
            }
        }
        _ => {}
    }

    match node_seccond.clone() {
        Ast::Types::Number(num) => {
            let number_b = num.val;
            if bin == '%' && select_binary == priority_first {
                let result = modulo(number_a, number_b);
                return calculattions_continue(num, result, select_binary);
            }

            if bin == '*' && select_binary == priority_first {
                let result = multiplication(number_a, number_b);
                return calculattions_continue(num, result, select_binary);
            }

            if bin == '/' && select_binary == priority_first {
                let result = division(number_a, number_b);
                return calculattions_continue(num, result, select_binary);
            }

            if bin == '-' && select_binary == priority_seccond {
                let result = minus(number_a, number_b);
                return calculattions_continue(num, result, select_binary);
            }

            if bin == '+' && select_binary == priority_seccond {
                let result = plus(number_a, number_b);
                return calculattions_continue(num, result, select_binary);
            }

            if bin == '<' && select_binary == priority_therd {
                match comparison_node {
                    Ast::Types::Binary(_) => {
                        let result = greater_than_equal(number_a, number_b);
                        return calculation_comparison_continue(num, result, select_binary);
                    }
                    _ => {
                        let result = greater_than(number_a, number_b);
                        return calculation_comparison_continue(num, result, select_binary);
                    }
                }
            }

            if bin == '>' && select_binary == priority_therd {
                match comparison_node {
                    Ast::Types::Binary(_) => {
                        let result = less_than_equal(number_a, number_b);
                        return calculation_comparison_continue(num, result, select_binary);
                    }
                    _ => {
                        let result = less_than(number_a, number_b);
                        return calculation_comparison_continue(num, result, select_binary);
                    }
                }
            }

            if bin == '=' && select_binary == priority_forth {
                match comparison_node {
                    Ast::Types::Binary(_) => {
                        let result = equivalence(number_a, number_b);
                        return calculation_comparison_continue(num, result, select_binary);
                    }

                    _ => {
                        panic!("error");
                    }
                }
            }

            if bin == '!'  && select_binary == priority_forth {
                match comparison_node {
                    Ast::Types::Binary(_) => {
                        let result = inequality(number_a, number_b);
                        return calculation_comparison_continue(num, result, select_binary);
                    }

                    _ => {
                        panic!("error");
                    }
                }
            }

            if !num.node.is_empty() {
                let result = calculattions(node_seccond.clone(), select_binary);

                let mut binarys = Ast::BinaryAST::new(bin);
                binarys.node.push(result);

                let mut numbers = Ast::NumberAST::new(number_a);
                numbers.node.push(Ast::Types::Binary(binarys));

                return Ast::Types::Number(numbers);
            }

            return numbers;
        }

        Ast::Types::Boolean(bools) => {
            let bool_b = bools.boolean;
            if bin == '=' && select_binary == priority_forth {
                match comparison_node {
                    Ast::Types::Binary(_) => {
                        let result = equivalence_bool(bool_a, bool_b);
                        return calculation_comparison_continue_bool(bools, result, select_binary);
                    }

                    _ => {
                        panic!("error");
                    }
                }
            }

            if bin == '!'  && select_binary == priority_forth {
                match comparison_node {
                    Ast::Types::Binary(_) => {
                        let result = inequality_bool(bool_a, bool_b);
                        return calculation_comparison_continue_bool(bools, result, select_binary);
                    }

                    _ => {
                        panic!("error");
                    }
                }
            }

            if bin == '&'  && select_binary == priority_fifth {
                match comparison_node {
                    Ast::Types::Binary(_) => {
                        let result = logical_and(bool_a, bool_b);
                        return calculation_comparison_continue_bool(bools, result, select_binary);
                    }

                    _ => {
                        panic!("error");
                    }
                }
            }

            if bin == '|'  && select_binary == priority_sixs {
                match comparison_node {
                    Ast::Types::Binary(_) => {
                        let result = logical_or(bool_a, bool_b);
                        return calculation_comparison_continue_bool(bools, result, select_binary);
                    }

                    _ => {
                        panic!("error");
                    }
                }
            }

            if !bools.node.is_empty() {
                let result = calculattions(node_seccond.clone(), select_binary);

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

    return numbers;
}

fn calculattions_continue(num: Ast::NumberAST, result: i64, select_binary: i64) -> Ast::Types {
    if num.node.is_empty() {
        let ast = Ast::NumberAST::new(result);
        return Ast::Types::Number(ast);
    }

    let mut number_ast = Ast::NumberAST::new(result);
    let node_number = num.node[0].clone();
    number_ast.node.push(node_number);

    let number_result = calculattions(Ast::Types::Number(number_ast), select_binary);
    return number_result;
}

fn calculation_comparison_continue(
    num: Ast::NumberAST,
    result: bool,
    select_binary: i64,
) -> Ast::Types {
    if num.node.is_empty() {
        let ast = Ast::BooleanAST::new(result);
        return Ast::Types::Boolean(ast);
    }

    let mut bool_ast = Ast::BooleanAST::new(result);
    let node_bool = num.node[0].clone();
    bool_ast.node.push(node_bool);

    let bool_result = calculattions(Ast::Types::Boolean(bool_ast), select_binary);
    return bool_result;
}

fn calculation_comparison_continue_bool(
    bools: Ast::BooleanAST,
    result: bool,
    select_binary: i64,
) -> Ast::Types {
    if bools.node.is_empty() {
        let ast = Ast::BooleanAST::new(result);
        return Ast::Types::Boolean(ast);
    }

    let mut bool_ast = Ast::BooleanAST::new(result);
    let node_bool = bools.node[0].clone();
    bool_ast.node.push(node_bool);

    let bool_result = calculattions(Ast::Types::Boolean(bool_ast), select_binary);
    return bool_result;
}

fn match_type(node: Ast::Types, next_node: Ast::Types) -> (Vec<i64>, Vec<Ast::Types>) {
    let mut numbers: Vec<i64> = Vec::new();
    let mut types: Vec<Ast::Types> = Vec::new();

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

fn greater_than(a: i64, b: i64) -> bool {
    a < b
}

fn greater_than_equal(a: i64, b: i64) -> bool {
    a <= b
}

fn less_than(a: i64, b: i64) -> bool {
    a > b
}

fn less_than_equal(a: i64, b: i64) -> bool {
    a >= b
}

fn equivalence(a: i64, b: i64) -> bool {
    a == b
}

fn equivalence_bool(a: bool, b: bool) -> bool {
    a == b
}

fn inequality(a: i64, b: i64) -> bool {
    a != b
}

fn inequality_bool(a: bool, b: bool) -> bool {
    a != b
}

fn logical_and(a: bool, b: bool) -> bool {
    a && b
}

fn logical_or(a: bool, b: bool) -> bool {
    a || b
}