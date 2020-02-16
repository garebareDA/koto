#[derive(Debug)]
pub enum Types {
    Number(NumberAST),
    Variable(VariableAST),
    Binary(BinaryAST),
    Call(CallAST),
}

#[derive(Debug)]
pub struct ExprAST {
    pub node: Vec<Types>,
}

impl ExprAST {
    pub fn new() -> ExprAST {
        ExprAST { node: Vec::new() }
    }
}

#[derive(Debug)]
pub struct NumberAST {
    pub val: i64,
    pub node: Vec<Types>,
}

impl NumberAST {
    pub fn new(val: i64) -> NumberAST {
        NumberAST {
            val:val,
            node: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct VariableAST {
    pub name: String,
    pub node: Vec<Types>,
}

impl VariableAST {
    pub fn new(string: &str) -> VariableAST {
        let string = string.to_string();
        VariableAST{
            name: string,
            node:Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct BinaryAST {
    pub op: char,
    pub node: Vec<Types>,
}

#[derive(Debug)]
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