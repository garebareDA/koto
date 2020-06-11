use super::super::ast::asts;
use super::error;
use super::interpreters;
use super::variable;
use std;

#[derive(Debug)]
pub struct Function {
    funcstions: Vec<Vec<asts::Types>>,
    inner: usize,
}

impl Function {
    pub fn new() -> Function {
        let mut fun = Function {
            funcstions: Vec::new(),
            inner: 0,
        };

        fun.funcstions.push(Vec::new());
        return fun;
    }

    pub fn last_remove(&mut self) {
        self.funcstions.remove(self.funcstions.len() - 1);
        self.out_fun();
    }

    pub fn vec_push(&mut self) {
        self.funcstions.push(Vec::new());
        self.in_fun();
    }

    fn in_fun(&mut self) {
        self.inner += 1;
    }

    fn out_fun(&mut self) {
        if self.inner == 0 {
            return;
        }
        self.inner -= 1;
    }

    pub fn push(&mut self, nodes: Vec<asts::Types>) {
        let mut index = 0;
        let len = nodes.len();
        loop {
            if index >= len {
                break;
            }

            let node = nodes[index].clone();
            match node {
                asts::Types::Function(_) => {
                    self.funcstions[self.inner].push(node);
                }
                _ => {}
            }
            index += 1;
        }
    }

    pub fn retrun_insert(&mut self, nodes: Vec<asts::Types>) -> Vec<asts::Types> {
        let mut index = 0;
        let mut inner: Vec<asts::Types> = Vec::new();
        let len = nodes.len();
        loop {
            if index >= len {
                break;
            }

            let node = nodes[index].clone();
            match node {
                asts::Types::Function(_) => {
                    inner.push(node);
                }
                _ => {}
            }
            index += 1;
        }

        return inner;
    }

    pub fn function_run(
        &mut self,
        call_ast: &asts::CallAST,
        variable: &mut variable::Variable,
    ) -> Option<asts::Types> {
        //引数に関数の一個上の部分まで
        //中身で判定

        let callee = &call_ast.callee;
        if callee == "print" {
            let value = &call_ast.argument[0];
            match value {
                asts::Types::Variable(var) => {
                    let (var_result, _) = variable.serch_variable(&var, self);
                    self.print_var(&var_result[0]);
                }

                asts::Types::Binary(_) => {
                    let result = interpreters::calculation(&value, variable, self);
                    self.print_var(&result);
                }

                _ => {
                    self.print_var(&value);
                }
            }
        }

        if callee == "stdin" {
            let mut console_in = String::new();
            std::io::stdin().read_line(&mut console_in).ok();
            let ast = asts::StringAST::new(&console_in);
            return Some(asts::Types::Strings(ast));
        }

        let serch_string = call_ast.callee.clone();
        let argument = call_ast.argument.clone();
        let mut functions = self.funcstions.clone();

        functions.reverse();
        for funcs in functions {
            for fun in funcs {
                match fun {
                    asts::Types::Function(fun) => {
                        if serch_string == fun.name {
                            return self.function(fun, &argument, variable);
                        }
                    }

                    _ => {}
                }
            }
        }

        return None;
    }

    fn function(
        &mut self,
        function: asts::FunctionAST,
        argument: &Vec<asts::Types>,
        vec_variable: &mut variable::Variable,
    ) -> Option<asts::Types> {
        vec_variable.vec_push();
        let mut index = 0;
        let function_arguments = &function.argument;
        let mut node = &function.node;

        for function_argument in function_arguments {
            match function_argument.clone() {
                asts::Types::Variable(mut variable) => {
                    if argument.is_empty() {
                        break;
                    }

                    if argument.len() <= index {
                        let err = error::Error::new(&function_argument.clone());
                        err.exit("argument error");
                    }

                    match &function.argument.clone()[index] {
                        asts::Types::Variable(var) => {
                            let types = &var.types;
                            let argument_type = &function.argument[index];
                            let continu = self.type_inspection(types, argument_type);
                            if !continu {
                                let err = error::Error::new(&function.argument.clone()[index]);
                                err.exit("argument type error");
                            }
                        }
                        _ => {
                            let err = error::Error::new(&function.argument.clone()[index]);
                            err.exit("argument type error");
                        }
                    }

                    let result = interpreters::calculation(&argument[index], vec_variable, self);
                    variable.node.push(result);
                    let variable = asts::Types::Variable(variable);
                    vec_variable.push(variable);
                }

                _ => {
                    let err = error::Error::new(&function_argument);
                    err.exit("argument error");
                }
            }

            index += 1;
        }

        let (_, result) = interpreters::scope(&mut node, vec_variable, self);
        let result_type = function.return_type;

        match &result {
            Some(types_result) => {
                let continu = self.type_inspection(&result_type, types_result);
                if !continu {
                    let err = error::Error::new(&function.argument.clone()[index]);
                    err.exit("argument type error");
                }
            }
            None => {}
        }

        vec_variable.last_remove();
        return result;
    }

    fn print_var(&self, var_result: &asts::Types) {
        match var_result {
            asts::Types::Strings(value) => {
                println!("{}", value.name);
            }
            asts::Types::Number(number) => {
                println!("{}", number.val);
            }
            asts::Types::Boolean(bools) => {
                println!("{}", bools.boolean);
            }
            _ => {
                let err = error::Error::new(&var_result);
                err.exit("print error");
            }
        }
    }

    fn type_inspection(
        &self,
        types: &Option<asts::VariableTypes>,
        argument_type: &asts::Types,
    ) -> bool {
        match types {
            Some(t) => match t {
                asts::VariableTypes::Bool => match argument_type {
                    asts::Types::Boolean(_) => true,
                    _ => false,
                },
                asts::VariableTypes::Int => match argument_type {
                    asts::Types::Number(_) => true,
                    _ => false,
                },
                asts::VariableTypes::Strings => match argument_type {
                    asts::Types::Strings(_) => true,
                    _ => false,
                },
            },
            None => false,
        }
    }
}
