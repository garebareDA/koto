#[derive(Debug, Clone)]
pub enum Types {
    Boolean(BooleanAST),
    Number(NumberAST),
    Strings(StringAST),
    Binary(BinaryAST),
    Call(CallAST),
    Variabel(VariableAST),
    If(IfAST),
    Scope(ScopeAST),
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
pub struct BooleanAST {
    pub boolean: bool,
    pub node: Vec<Types>,
}

impl BooleanAST {
    pub fn new(boolean: bool) -> BooleanAST {
        BooleanAST{
            boolean:boolean,
            node: Vec::new(),
        }
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
pub struct IfAST {
    pub judge:Vec<Types>,
    pub node: Vec<Types>,
}

impl IfAST{
    pub fn new(judge: Vec<Types>) -> IfAST {
        IfAST{
            judge: judge,
            node:Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForAST {
    pub init_var: Types,
    pub conditions: Types,
    pub increase : Types,
    pub node: Vec<Types>
}

impl ForAST {
    pub fn new(init:Types, cond:Types, inc:Types) -> ForAST {
        ForAST{
            init_var: init,
            conditions: cond,
            increase : inc,
            node: Vec::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScopeAST {
    pub scope: char,
}

impl ScopeAST {
    pub fn new(scope: char) -> ScopeAST {
        ScopeAST{
            scope:scope
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