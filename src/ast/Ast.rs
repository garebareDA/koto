#[derive(Debug, Clone)]
pub enum Types {
    Boolean(BooleanAST),
    Number(NumberAST),
    Strings(StringAST),
    Binary(BinaryAST),
    Call(CallAST),
    Function(FunctionAST),
    Variable(VariableAST),
    Vector(VectorAST),
    If(IfAST),
    For(ForAST),
    Retrun(RetrunAST),
    Scope(ScopeAST),
    Square(SquareAST),
    Comma(CommaAST),
    Parent(ParenthesesAST),
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
    pub argument: Vec<Types>,
    pub node: Vec<Types>,
}

impl CallAST {
    pub fn new(string: &str) -> CallAST {
        let string = string.to_string();
        CallAST {
            callee: string,
            argument: Vec::new(),
            node: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionAST {
    pub name: String,
    pub argument: Vec<Types>,
    pub node: Vec<Types>,
}

impl FunctionAST {
    pub fn new(string: &str) -> FunctionAST {
        let string = string.to_string();
        FunctionAST{
            name:string,
            argument: Vec::new(),
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
pub struct VectorAST {
    pub node: Vec<Types>,
}

impl VectorAST {
    pub fn new() -> VectorAST {
        VectorAST{
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
    pub init_var: Vec<Types>,
    pub node: Vec<Types>
}

impl ForAST {
    pub fn new(init:Types, cond:Types, inc:Types) -> ForAST {
        let mut for_types = Vec::new();
        for_types.push(init);
        for_types.push(cond);
        for_types.push(inc);

        ForAST{
            init_var: for_types,
            node: Vec::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct RetrunAST {
    pub node: Vec<Types>,
}

impl RetrunAST {
    pub fn new() -> RetrunAST {
        RetrunAST{
            node:Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommaAST {
    pub comma: char
}

impl CommaAST {
    pub fn new(comma: char) -> CommaAST {
        CommaAST {
            comma: comma
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParenthesesAST {
    pub parent:char,
}

impl ParenthesesAST {
    pub fn new(parent:char) -> ParenthesesAST {
        ParenthesesAST{
            parent: parent
        }
    }
}

#[derive(Debug, Clone)]
pub struct SquareAST {
    pub square: char,
}

impl SquareAST {
    pub fn new(square: char) -> SquareAST {
        SquareAST{
            square:square
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