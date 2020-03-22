use super::super::ast::Ast;
use super::Interpreter;
use super::Variable;

pub struct function {
    funcstions: Vec<Vec<Ast::Types>>,
    inner: usize,
}

impl function {
    pub fn new() -> function {
        let mut fun = function {
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

    pub fn push(&mut self, nodes: Vec<Ast::Types>) {
        let mut index = 0;
        let len = nodes.len();
        loop {
            if index >= len {
                break;
            }

            let node = nodes[index].clone();
            match node {
                Ast::Types::Function(_) => {
                    self.funcstions[self.inner].push(node);
                }
                _ => {}
            }
            index += 1;
        }
    }

    pub fn function_run(&mut self, call_ast: &Ast::CallAST, variable: &mut Variable::Variable) ->Option<Ast::Types> {
        let callee = call_ast.callee.clone();
        if callee == "print" {
            let value = &call_ast.node[0];
            match value {
                Ast::Types::Variable(var) => {
                    let var_result = variable.serch_variable(&var.name, self);
                    self.print_var(&var_result);
                }
                _ => {
                    self.print_var(&value);
                }
            }
        }

        let serch_string = call_ast.callee.clone();
        let argument = call_ast.node.clone();
        let mut functions = self.funcstions.clone();
        functions.reverse();

        for funcs in functions {
            for fun in funcs {
                match fun {
                    Ast::Types::Function(fun) => {
                        if serch_string == fun.name {
                            return self.function(fun.argument, &argument, fun.node, variable);
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
        function_arguments: Vec<Ast::Types>,
        argument: &Vec<Ast::Types>,
        mut node: Vec<Ast::Types>,
        vec_variable: &mut Variable::Variable,
    ) -> Option<Ast::Types> {
        vec_variable.vec_push();
        let mut index = 0;

        for function_argument in function_arguments {
            match function_argument {
                Ast::Types::Variable(mut variable) => {
                    variable.node.push(argument[index].clone());
                    let variable = Ast::Types::Variable(variable);
                    vec_variable.push(variable);
                }

                _ => {}
            }

            index += 1;
        }

        let (_, result) = Interpreter::scope(&mut node, vec_variable, self);
        vec_variable.last_remove();
        return result;
    }

    fn print_var(&self, var_result: &Ast::Types) {
        match var_result {
            Ast::Types::Strings(value) => {
                println!("{}", value.name);
            }
            Ast::Types::Number(number) => {
                println!("{}", number.val);
            }
            Ast::Types::Boolean(bools) => {
                println!("{}", bools.boolean);
            }
            _ => {}
        }
    }
}
