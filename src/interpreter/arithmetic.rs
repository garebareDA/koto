use super::super::ast::asts;
use super::error;

pub fn common(bin: asts::BinaryAST) -> asts::Types {
    let op = bin.op;
    let node = bin.node[0].clone();
    let mut next_node = bin.node[1].clone();

    if bin.op == '-' || bin.op == '+' {
        match &next_node {
            asts::Types::Binary(bin) => match comparison_operator(bin.op, &node) {
                Some(num) => {
                    return asts::Types::Number(asts::NumberAST::new(num));
                }
                None => {}
            },

            _ => {}
        }
    }

    if op == '%' {
        let (numbers, types) = match_type(node.clone(), next_node.clone());
        let result = modulo(numbers[0], numbers[1]);
        let mut ast = asts::NumberAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = asts::Types::Number(ast);
        } else {
            next_node = asts::Types::Number(ast);
        }
    }

    if op == '*' {
        let (numbers, types) = match_type(node.clone(), next_node.clone());
        let result = multiplication(numbers[0], numbers[1]);
        let mut ast = asts::NumberAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = asts::Types::Number(ast);
        } else {
            next_node = asts::Types::Number(ast);
        }
    }

    if op == '/' {
        let (numbers, types) = match_type(node.clone(), next_node.clone());
        let result = division(numbers[0], numbers[1]);
        let mut ast = asts::NumberAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = asts::Types::Number(ast);
        } else {
            next_node = asts::Types::Number(ast);
        }
    }

    next_node = calculattions(next_node, 1);

    if op == '-' {
        let (numbers, types) = match_type(node.clone(), next_node.clone());
        let result = minus(numbers[0], numbers[1]);
        let mut ast = asts::NumberAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = asts::Types::Number(ast);
        } else {
            next_node = asts::Types::Number(ast);
        }
    }

    if op == '+' {
        let (numbers, strings, types) = match_type_possible(node.clone(), next_node.clone());
        if strings.len() == 0 {
            let result = plus(numbers[0], numbers[1]);
            let mut ast = asts::NumberAST::new(result);
            if !types.is_empty() {
                ast.node.push(types[0].clone());
                next_node = asts::Types::Number(ast);
            } else {
                next_node = asts::Types::Number(ast);
            }
        } else {
            let result = concatenation(&strings[0], &strings[1]);
            let mut ast = asts::StringAST::new(&result);
            if !types.is_empty() {
                ast.node.push(types[0].clone());
                next_node = asts::Types::Strings(ast);
            } else {
                next_node = asts::Types::Strings(ast);
            }
        }
    }

    next_node = calculattions(next_node, 2);

    if op == '<' {
        let (numbers, types) = match_type(node.clone(), next_node.clone());
        let result: bool;

        if bin.node.len() == 3 {
            result = greater_than_equal(numbers[0], numbers[1]);
        } else {
            result = greater_than(numbers[0], numbers[1]);
        }

        let mut ast = asts::BooleanAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = asts::Types::Boolean(ast);
        } else {
            next_node = asts::Types::Boolean(ast);
        }
    }

    if op == '>' {
        let (numbers, types) = match_type(node.clone(), next_node.clone());
        let result: bool;

        if bin.node.len() == 3 {
            result = less_than_equal(numbers[0], numbers[1]);
        } else {
            result = less_than(numbers[0], numbers[1]);
        }

        let mut ast = asts::BooleanAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = asts::Types::Boolean(ast);
        } else {
            next_node = asts::Types::Boolean(ast);
        }
    }

    next_node = calculattions(next_node, 3);

    if op == '=' {
        let (numbers, strings, types) = match_type_possible(node.clone(), next_node.clone());
        let mut result = false;

        if strings.len() == 0 && bin.node.len() == 3 {
            result = equivalence(numbers[0], numbers[1]);
        } else if bin.node.len() == 3 {
            result = equivalence_string(&strings[0], &strings[1]);
        } else {
            let err = error::Error::new(&next_node);
            err.exit("Comparison operator error");
        }

        let mut ast = asts::BooleanAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = asts::Types::Boolean(ast);
        } else {
            next_node = asts::Types::Boolean(ast);
        }
    }

    if op == '!' {
        let (numbers, strings, types) = match_type_possible(node.clone(), next_node.clone());
        let mut result = false;

        if strings.len() == 0 && bin.node.len() == 3 {
            result = inequality(numbers[0], numbers[1]);
        } else if bin.node.len() == 3 {
            result = inequality_string(&strings[0], &strings[1]);
        } else {
            let err = error::Error::new(&next_node);
            err.exit("Comparison operator error");
        }

        let mut ast = asts::BooleanAST::new(result);
        if !types.is_empty() {
            ast.node.push(types[0].clone());
            next_node = asts::Types::Boolean(ast);
        } else {
            next_node = asts::Types::Boolean(ast);
        }
    }

    next_node = calculattions(next_node, 4);
    next_node = calculattions(next_node, 5);
    next_node = calculattions(next_node, 6);
    return next_node;
}

fn calculattions(numbers: asts::Types, select_binary: i64) -> asts::Types {
    let mut number_a = 0;
    let mut bool_a = false;
    let mut string_a = String::new();
    let mut bin = ' ';

    let mut node_first = asts::Types::Error(asts::ErrorAST::new("variable error"));
    let mut node_seccond = asts::Types::Error(asts::ErrorAST::new("variable error"));
    let mut comparison_node = asts::Types::Error(asts::ErrorAST::new("variable error"));

    let priority_first = 1;
    let priority_seccond = 2;
    let priority_therd = 3;
    let priority_forth = 4;
    let priority_fifth = 5;
    let priority_sixs = 6;

    match numbers.clone() {
        asts::Types::Number(num) => {
            if num.node.is_empty() {
                return numbers;
            }
            number_a = num.val;
            node_first = num.node[0].clone();
        }

        asts::Types::Boolean(bools) => {
            if bools.node.is_empty() {
                return numbers;
            }
            bool_a = bools.boolean;
            node_first = bools.node[0].clone();
        }

        asts::Types::Strings(string) => {
            if string.node.is_empty() {
                return numbers;
            }
            string_a = string.name;
            node_first = string.node[0].clone();
        }

        _ => {
            let err = error::Error::new(&numbers);
            err.exit("number error");
        }
    }

    match node_first.clone() {
        asts::Types::Binary(binary) => {
            bin = binary.op;
            let len = binary.node.len();

            if len == 1 {
                node_seccond = binary.node[0].clone();
            } else if len >= 2 {
                comparison_node = binary.node[0].clone();
                node_seccond = binary.node[1].clone();
            } else {
                let err = error::Error::new(&node_first);
                err.exit("number operater error");
            }
        }
        _ => {}
    }

    match node_seccond.clone() {
        asts::Types::Number(num) => {
            let number_b = num.val;
            if bin == '%' && select_binary == priority_first {
                let result = modulo(number_a, number_b);
                let modulo_reslut = asts::Types::Number(asts::NumberAST::new(result));
                return calculations_continue(num.node, modulo_reslut, select_binary);
            }

            if bin == '*' && select_binary == priority_first {
                let result = multiplication(number_a, number_b);
                let multiplication_reslut = asts::Types::Number(asts::NumberAST::new(result));
                return calculations_continue(num.node, multiplication_reslut, select_binary);
            }

            if bin == '/' && select_binary == priority_first {
                let result = division(number_a, number_b);
                let division_reslut = asts::Types::Number(asts::NumberAST::new(result));
                return calculations_continue(num.node, division_reslut, select_binary);
            }

            if bin == '-' && select_binary == priority_seccond {
                let result = minus(number_a, number_b);
                let minus_result = asts::Types::Number(asts::NumberAST::new(result));
                return calculations_continue(num.node, minus_result, select_binary);
            }

            if bin == '+' && select_binary == priority_seccond {
                if string_a.len() > 0 {
                    let result = &concatenation(&string_a, &number_b.to_string());
                    let concatenation_result = asts::Types::Strings(asts::StringAST::new(result));
                    return calculations_continue(num.node, concatenation_result, select_binary);
                }
                let result = plus(number_a, number_b);
                let plus_result = asts::Types::Number(asts::NumberAST::new(result));
                return calculations_continue(num.node, plus_result, select_binary);
            }

            if bin == '<' && select_binary == priority_therd {
                match comparison_node {
                    asts::Types::Binary(_) => {
                        let result = greater_than_equal(number_a, number_b);
                        let greater_than_equal_result =
                            asts::Types::Boolean(asts::BooleanAST::new(result));
                        return calculations_continue(
                            num.node,
                            greater_than_equal_result,
                            select_binary,
                        );
                    }
                    _ => {
                        let result = greater_than(number_a, number_b);
                        let greater_than_result =
                            asts::Types::Boolean(asts::BooleanAST::new(result));
                        return calculations_continue(
                            num.node,
                            greater_than_result,
                            select_binary,
                        );
                    }
                }
            }

            if bin == '>' && select_binary == priority_therd {
                match comparison_node {
                    asts::Types::Binary(_) => {
                        let result = less_than_equal(number_a, number_b);
                        let less_than_equal_result =
                            asts::Types::Boolean(asts::BooleanAST::new(result));
                        return calculations_continue(
                            num.node,
                            less_than_equal_result,
                            select_binary,
                        );
                    }
                    _ => {
                        let result = less_than(number_a, number_b);
                        let less_than_result = asts::Types::Boolean(asts::BooleanAST::new(result));
                        return calculations_continue(num.node, less_than_result, select_binary);
                    }
                }
            }

            if bin == '=' && select_binary == priority_forth {
                match comparison_node {
                    asts::Types::Binary(_) => {
                        if string_a.len() > 0 {
                            let result = equivalence_string(&string_a, &number_b.to_string());
                            let equivalence_string_result =
                                asts::Types::Boolean(asts::BooleanAST::new(result));
                            return calculations_continue(
                                num.node,
                                equivalence_string_result,
                                select_binary,
                            );
                        }
                        let result = equivalence(number_a, number_b);
                        let equivalence_result =
                            asts::Types::Boolean(asts::BooleanAST::new(result));
                        return calculations_continue(num.node, equivalence_result, select_binary);
                    }

                    _ => {
                        let err = error::Error::new(&comparison_node);
                        err.exit("number error");
                    }
                }
            }

            if bin == '!' && select_binary == priority_forth {
                match comparison_node {
                    asts::Types::Binary(_) => {
                        if string_a.len() > 0 {
                            let result = inequality_string(&string_a, &number_b.to_string());
                            let inequality_string_result = asts::Types::Boolean(asts::BooleanAST::new(result));
                            return calculations_continue(
                                num.node,
                                inequality_string_result,
                                select_binary,
                            );
                        }
                        let result = inequality(number_a, number_b);
                        let inequality_result = asts::Types::Boolean(asts::BooleanAST::new(result));
                        return calculations_continue(num.node, inequality_result, select_binary);
                    }

                    _ => {
                        let err = error::Error::new(&comparison_node);
                        err.exit("number error");
                    }
                }
            }

            if !num.node.is_empty() {
                let result = calculattions(node_seccond.clone(), select_binary);

                let mut binarys = asts::BinaryAST::new(bin);

                match comparison_node {
                    asts::Types::Binary(_) => {
                        binarys.node.push(comparison_node);
                    }
                    _ => {}
                }

                binarys.node.push(result);
                let inner = (number_a, bool_a, &string_a);
                return ast_return(binarys, numbers, inner);
            }

            return numbers;
        }

        asts::Types::Boolean(bools) => {
            let bool_b = bools.boolean;
            if bin == '=' && select_binary == priority_forth {
                match comparison_node {
                    asts::Types::Binary(_) => {
                        if string_a.len() > 0 {
                            let result = equivalence_string(&string_a, &bool_b.to_string());
                            let equivalence_string_result = asts::Types::Boolean(asts::BooleanAST::new(result));
                            return calculations_continue(
                                bools.node,
                                equivalence_string_result,
                                select_binary,
                            );
                        }
                        let result = equivalence_bool(bool_a, bool_b);
                        let equivalence_bool_result = asts::Types::Boolean(asts::BooleanAST::new(result));
                        return calculations_continue(bools.node, equivalence_bool_result, select_binary);
                    }

                    _ => {
                        let err = error::Error::new(&comparison_node);
                        err.exit("Comparison operator error")
                    }
                }
            }

            if bin == '!' && select_binary == priority_forth {
                match comparison_node {
                    asts::Types::Binary(_) => {
                        if string_a.len() > 0 {
                            let result = inequality_string(&string_a, &bool_b.to_string());
                            let inequality_string_result = asts::Types::Boolean(asts::BooleanAST::new(result));
                            return calculations_continue(
                                bools.node,
                                inequality_string_result,
                                select_binary,
                            );
                        }
                        let result = inequality_bool(bool_a, bool_b);
                        let inequality_bool_result = asts::Types::Boolean(asts::BooleanAST::new(result));
                        return calculations_continue(bools.node, inequality_bool_result, select_binary);
                    }

                    _ => {
                        let err = error::Error::new(&comparison_node);
                        err.exit("Comparison operator error")
                    }
                }
            }

            if bin == '&' && select_binary == priority_fifth {
                match comparison_node {
                    asts::Types::Binary(_) => {
                        let result = logical_and(bool_a, bool_b);
                        let logical_and_result = asts::Types::Boolean(asts::BooleanAST::new(result));
                        return calculations_continue(bools.node, logical_and_result, select_binary);
                    }

                    _ => {
                        let err = error::Error::new(&comparison_node);
                        err.exit("Comparison operator error")
                    }
                }
            }

            if bin == '|' && select_binary == priority_sixs {
                match comparison_node {
                    asts::Types::Binary(_) => {
                        let result = logical_or(bool_a, bool_b);
                        let logical_or_result = asts::Types::Boolean(asts::BooleanAST::new(result));
                        return calculations_continue(bools.node, logical_or_result, select_binary);
                    }

                    _ => {
                        let err = error::Error::new(&comparison_node);
                        err.exit("Comparison operator error")
                    }
                }
            }

            if !bools.node.is_empty() {
                let result = calculattions(node_seccond.clone(), select_binary);

                let mut binarys = asts::BinaryAST::new(bin);

                match comparison_node {
                    asts::Types::Binary(_) => {
                        binarys.node.push(comparison_node);
                    }
                    _ => {
                        let err = error::Error::new(&comparison_node);
                        err.exit("Comparison operator error")
                    }
                }

                binarys.node.push(result);
                let inner = (number_a, bool_a, &string_a);
                return ast_return(binarys, numbers, inner);
            }

            return numbers;
        }

        asts::Types::Strings(string) => {
            let string_b = &string.name;
            if bin == '+' && select_binary == priority_seccond {
                match numbers.clone() {
                    asts::Types::Strings(_) => {
                        let result = &concatenation(&string_a, string_b);
                        let concatenation_result = asts::Types::Strings(asts::StringAST::new(result));
                        return calculations_continue(string.node, concatenation_result, select_binary);
                    }

                    asts::Types::Number(_) => {
                        let result = &concatenation(&number_a.to_string(), string_b);
                        let concatenation_result = asts::Types::Strings(asts::StringAST::new(result));
                        return calculations_continue(string.node, concatenation_result, select_binary);
                    }

                    asts::Types::Boolean(_) => {
                        let result = &concatenation(&bool_a.to_string(), string_b);
                        let concatenation_result = asts::Types::Strings(asts::StringAST::new(result));
                        return calculations_continue(string.node, concatenation_result, select_binary);
                    }

                    _ => {}
                }
            }

            if bin == '=' && select_binary == priority_forth {
                match comparison_node {
                    asts::Types::Binary(_) => match numbers.clone() {
                        asts::Types::Strings(_) => {
                            let result = equivalence_string(&string_a, string_b);
                            let equivalence_string_result = asts::Types::Boolean(asts::BooleanAST::new(result));
                            return calculations_continue(string.node, equivalence_string_result , select_binary);
                        }

                        asts::Types::Number(_) => {
                            let result = equivalence_string(&number_a.to_string(), string_b);
                            let equivalence_string_result = asts::Types::Boolean(asts::BooleanAST::new(result));
                            return calculations_continue(string.node, equivalence_string_result , select_binary);
                        }

                        asts::Types::Boolean(_) => {
                            let result = equivalence_string(&bool_a.to_string(), string_b);
                            let equivalence_string_result = asts::Types::Boolean(asts::BooleanAST::new(result));
                            return calculations_continue(string.node, equivalence_string_result , select_binary);
                        }

                        _ => {}
                    },
                    _ => {}
                }
            }

            if bin == '!' && select_binary == priority_forth {
                match comparison_node {
                    asts::Types::Binary(_) => match numbers.clone() {
                        asts::Types::Strings(_) => {
                            let result = inequality_string(&string_a, string_b);
                            let inequality_string_result = asts::Types::Boolean(asts::BooleanAST::new(result));
                            return calculations_continue(string.node, inequality_string_result, select_binary);
                        }

                        asts::Types::Number(_) => {
                            let result = inequality_string(&number_a.to_string(), string_b);
                            let inequality_string_result = asts::Types::Boolean(asts::BooleanAST::new(result));
                            return calculations_continue(string.node, inequality_string_result, select_binary);
                        }

                        asts::Types::Boolean(_) => {
                            let result = inequality_string(&bool_a.to_string(), string_b);
                            let inequality_string_result = asts::Types::Boolean(asts::BooleanAST::new(result));
                            return calculations_continue(string.node, inequality_string_result, select_binary);
                        }

                        _ => {}
                    },
                    _ => {}
                }
                if !string.node.is_empty() {
                    let result = calculattions(node_seccond.clone(), select_binary);
                    let mut binarys = asts::BinaryAST::new(bin);

                    match comparison_node {
                        asts::Types::Binary(_) => {
                            binarys.node.push(comparison_node);
                        }
                        _ => {}
                    }
                    binarys.node.push(result);
                    let inner = (number_a, bool_a, &string_a);
                    return ast_return(binarys, numbers, inner);
                }
                return numbers;
            }
        }

        _ => {
            let err = error::Error::new(&comparison_node);
            err.exit("operator or Number error")
        }
    }

    return numbers;
}

fn calculations_continue(
    node: Vec<asts::Types>,
    result: asts::Types,
    select_binary: i64,
) -> asts::Types {
    if node.is_empty() {
        return result;
    }

    match result {
        asts::Types::Number(mut number) => {
            number.node.push(node[0].clone());
            let number_result = calculattions(asts::Types::Number(number), select_binary);
            return number_result;
        }

        asts::Types::Boolean(mut bools) => {
            bools.node.push(node[0].clone());
            let bools_result = calculattions(asts::Types::Boolean(bools), select_binary);
            return bools_result;
        }

        asts::Types::Strings(mut strings) => {
            strings.node.push(node[0].clone());
            let strings_result = calculattions(asts::Types::Strings(strings), select_binary);
            return strings_result;
        }

        _ => {asts::Types::Error(asts::ErrorAST::new("Calculattions Error"))}
    }
}

fn match_type(node: asts::Types, next_node: asts::Types) -> (Vec<i64>, Vec<asts::Types>) {
    let mut numbers: Vec<i64> = Vec::new();
    let mut types: Vec<asts::Types> = Vec::new();

    match node {
        asts::Types::Number(num) => {
            numbers.push(num.val);
        }

        _ => {
            let err = error::Error::new(&node);
            err.exit("not a number error");
        }
    }

    match next_node {
        asts::Types::Number(num) => {
            numbers.push(num.val);
            if !num.node.is_empty() {
                types.push(num.node[0].clone());
            }
        }

        _ => {
            let err = error::Error::new(&next_node);
            err.exit("number node error");
        }
    }

    return (numbers, types);
}

fn match_type_possible(
    node: asts::Types,
    next_node: asts::Types,
) -> (Vec<i64>, Vec<String>, Vec<asts::Types>) {
    let mut numbers: Vec<i64> = Vec::new();
    let mut strings: Vec<String> = Vec::new();
    let mut types: Vec<asts::Types> = Vec::new();

    match node {
        asts::Types::Number(num) => {
            numbers.push(num.val);
        }

        asts::Types::Strings(string) => {
            let name = string.name;
            strings.push(name);
        }

        _ => {
            let err = error::Error::new(&node);
            err.exit("not a number error");
        }
    }

    match &next_node {
        asts::Types::Number(num) => {
            if strings.len() == 0 {
                numbers.push(num.val);
            } else {
                strings.push(num.val.to_string());
            }

            if !num.node.is_empty() {
                types.push(num.node[0].clone());
            }
        }

        asts::Types::Strings(string) => {
            if numbers.len() > 0 {
                strings.push(numbers[0].to_string());
                numbers.clear();
            }
            let name = &string.name;
            strings.push(name.clone());
            if !string.node.is_empty() {
                types.push(string.node[0].clone());
            }
        }

        _ => {
            let err = error::Error::new(&next_node);
            err.exit("number node error");
        }
    }

    return (numbers, strings, types);
}

fn comparison_operator(op: char, number: &asts::Types) -> Option<i64> {
    match number {
        asts::Types::Number(num) => {
            if op == '+' {
                let result = plus(num.val, 1);
                return Some(result);
            } else if op == '-' {
                let result = minus(num.val, 1);
                return Some(result);
            }
        }

        _ => {
            let err = error::Error::new(&number);
            err.exit("not a number error");
        }
    }
    return None;
}

fn ast_return(binarys:asts::BinaryAST, numbers:asts::Types, inner:(i64, bool, &String)) -> asts::Types {
    let mut ast_retruns = asts::Types::Error(asts::ErrorAST::new("variable error"));
    let (number_a, bool_a, string_a) = inner;

    match numbers.clone() {
        asts::Types::Number(_) => {
            let mut numbers = asts::NumberAST::new(number_a);
            numbers.node.push(asts::Types::Binary(binarys));
            let ast_return = asts::Types::Number(numbers);
            ast_retruns = ast_return
        }
        asts::Types::Boolean(_) => {
            let mut booleans = asts::BooleanAST::new(bool_a);
            booleans.node.push(asts::Types::Binary(binarys));
            let ast_return = asts::Types::Boolean(booleans);
            ast_retruns = ast_return
        }
        asts::Types::Strings(_) => {
            let mut strings = asts::StringAST::new(string_a);
            strings.node.push(asts::Types::Binary(binarys));
            let ast_retrun = asts::Types::Strings(strings);
            ast_retruns = ast_retrun;
        }
        _ => {
            let err = error::Error::new(&numbers);
            err.exit("number error");
        }
    }
    return ast_retruns;
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

fn concatenation(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}

fn equivalence_string(a: &str, b: &str) -> bool {
    a == b
}

fn inequality_string(a: &str, b: &str) -> bool {
    a != b
}