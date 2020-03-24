use super::super::ast::Ast;
use super::Function;
use super::Interpreter;

pub struct Variable {
    variables: Vec<Vec<Ast::Types>>,
    inner: usize,
}

impl Variable {
    pub fn new() -> Variable {
        let mut var = Variable {
            variables: Vec::new(),
            inner: 0,
        };

        var.variables.push(Vec::new());
        return var;
    }

    pub fn last_remove(&mut self) {
        self.variables.remove(self.variables.len() - 1);
        self.out_var();
    }

    pub fn vec_push(&mut self) {
        self.variables.push(Vec::new());
        self.in_var();
    }

    pub fn push(&mut self, var: Ast::Types) {
        self.variables[self.inner].push(var);
    }

    fn in_var(&mut self) {
        self.inner += 1;
    }

    fn out_var(&mut self) {
        if self.inner == 0 {
            return;
        }
        self.inner -= 1;
    }

    pub fn variable(
        &mut self,
        var: Ast::Types,
        vec_function: &mut Function::function,
    ) -> Ast::Types {
        match var {
            Ast::Types::Binary(_) => {
                return Interpreter::calculation(var, self, vec_function);
            }

            _ => return var,
        }
    }

    pub fn serch_variable(
        &mut self,
        serch: &Ast::VariableAST,
        vec_function: &mut Function::function,
    ) -> Ast::Types {
        let mut variable_retrun = Ast::Types::Error(Ast::ErrorAST::new("Vairable Error"));
        let var_vec = self.variables.clone();
        let serch = serch.clone();
        let serch_word = serch.name;

        for vars in var_vec {
            for var in vars {
                let mut in_var = Ast::VariableAST::new("");

                match var {
                    Ast::Types::Variable(in_vars) => {
                        in_var = in_vars;
                    }
                    _ => {}
                }

                if in_var.name == serch_word {
                    match in_var.node[0].clone() {
                        Ast::Types::Variable(var) => {
                            variable_retrun = self.serch_variable(&var, vec_function);
                        }

                        Ast::Types::Call(_) => {
                            let (_, result) =
                                Interpreter::run_judg(&in_var.node[0], self, vec_function);
                            match result {
                                Some(somes) => {
                                    variable_retrun = somes;
                                }

                                None => {}
                            }
                        }

                        Ast::Types::Vector(vector) => match &serch.index {
                            Some(index) => match &index[0] {
                                Ast::Types::Number(num) => {
                                    let vector_num = num.val;
                                    let var = &vector.node[vector_num as usize];
                                    variable_retrun = var.clone();
                                }

                                _ => {}
                            },
                            None => {}
                        },

                        _ => {
                            variable_retrun = in_var.node[0].clone();
                        }
                    }
                }
            }
        }
        return variable_retrun;
    }

    pub fn variables_allocation(
        &mut self,
        serch: Vec<Ast::Types>,
        vec_function: &mut Function::function,
    ) -> Vec<Ast::Types> {
        let mut ast_vec: Vec<Ast::Types> = Vec::new();
        for node in serch {
            match node {
                Ast::Types::Binary(mut bin) => {
                    if !bin.node.is_empty() {
                        let vec = self.variables_allocation(bin.node.clone(), vec_function);
                        bin.node = vec;
                    }
                    ast_vec.push(Ast::Types::Binary(bin));
                }
                Ast::Types::Number(mut num) => {
                    if !num.node.is_empty() {
                        let vec = self.variables_allocation(num.node.clone(), vec_function);
                        num.node = vec;
                    }
                    ast_vec.push(Ast::Types::Number(num));
                }
                Ast::Types::Variable(var) => {
                    let mut vec: Vec<Ast::Types> = Vec::new();
                    if !var.node.is_empty() {
                        vec = self.variables_allocation(var.node.clone(), vec_function);
                    }
                    let serch_result = self.serch_variable(&var, vec_function);
                    match serch_result {
                        Ast::Types::Number(mut num) => {
                            num.node = vec;
                            ast_vec.push(Ast::Types::Number(num));
                        }
                        _ => {}
                    }
                }

                Ast::Types::Call(call) => {
                    let result = vec_function.function_run(&call, self);
                    match result {
                        Some(result) => {
                            ast_vec.push(result);
                        }

                        None => {}
                    }
                }
                _ => {}
            }
        }
        return ast_vec;
    }
}
