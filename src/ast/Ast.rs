pub enum Types {
    Number(NumberAST),
    Variable(VariableAST),
    Binary(BinaryAST),
    Call(CallAST),
}

pub struct ExprAST {
    pub node: Vec<Types>,
}

impl ExprAST {
    pub fn new() -> ExprAST {
        ExprAST { node: Vec::new() }
    }
}

pub struct NumberAST {
    pub val: i64,
    pub node: Vec<Types>,
}

pub struct VariableAST {
    pub name: String,
    pub node: Vec<Types>,
}

pub struct BinaryAST {
    pub op: char,
    pub node: Vec<Types>,
}

pub struct CallAST {
    pub callee: String,
    pub node: Vec<Types>,
}

impl CallAST {
    pub fn new(string: &str) -> CallAST {
        let string = string.to_string();
        CallAST {
            callee: string,
            node: Vec::new(),
        }
    }
}
