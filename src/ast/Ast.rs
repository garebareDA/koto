#[derive(Debug, Clone)]
pub enum Types {
    Number(NumberAST),
    Variable(VariableAST),
    Binary(BinaryAST),
    Call(CallAST),
    End(EndAST),
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct BinaryAST {
    pub op: char,
    pub node: Vec<Types>,
}

impl BinaryAST {
    pub fn new(binary: char) -> BinaryAST {
        BinaryAST {
            op: binary,
            node: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct EndAST {
    pub end: String,
}

impl EndAST {
    pub fn new() -> Self {
        EndAST {
            end: "end".to_string(),
        }
    }
}