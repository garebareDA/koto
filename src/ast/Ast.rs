pub enum Types {
    Number(NumberAST),
    Variable(VariableAST),
    Binary(BinaryAST),
    Call(CallAST),
}

pub struct ExprAST {
    pub node: Vec<Types>
}

pub struct NumberAST {
    pub val:i64,
    pub node: Vec<Types>
}

pub struct VariableAST {
    pub name: String,
    pub node: Vec<Types>
}

pub struct BinaryAST {
    pub op:char,
    pub node: Vec<Types>
}

pub struct CallAST {
    pub callee: String,
    pub node: Vec<Types>
}