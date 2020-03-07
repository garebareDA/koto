#[derive(Debug, Clone)]
//TODO 取り出すメソッドを追加する
//TODO エラーのenumを追加する
pub enum Types {
    Number(NumberAST),
    Strings(StringAST),
    Binary(BinaryAST),
    Call(CallAST),
    Variabel(VariableAST),
    End(EndAST),
    Error(ErrorAST)
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
pub struct StringAST {
    pub name: String,
    pub node: Vec<Types>,
}

impl StringAST {
    pub fn new(string: &str) -> StringAST {
        let string = string.to_string();
        StringAST{
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
pub struct VariableAST{
    pub name: String,
    pub node: Vec<Types>,
}

impl VariableAST {
    pub fn new(string: &str) -> VariableAST {
        let string = string.to_string();
        VariableAST {
            name: string,
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

#[derive(Debug, Clone)]
pub struct ErrorAST {
    pub error: String,
}

impl ErrorAST {
    pub fn new(error: &str) -> Self{
        ErrorAST{
            error:error.to_string(),
        }
    }
}