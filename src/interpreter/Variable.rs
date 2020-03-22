use super::super::ast::Ast;
use super::Interpreter;
use super::Function;

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

    pub fn push(&mut self, var:Ast::Types) {
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

    pub fn variable(&mut self, var: Ast::Types, vec_function: &mut Function::function) -> Ast::Types {
        match var {
            Ast::Types::Binary(_) => {
                return Interpreter::calculation(var, self, vec_function);
            }

            _ => return var,
        }
    }

    pub fn serch_variable(&mut self, serch: &str, vec_function: &mut Function::function) -> Ast::Types {
        let mut variable_retrun = Ast::Types::Error(Ast::ErrorAST::new("Vairable Error"));
        for vars in self.variables.clone() {
            for var in vars {
                match var {
                    Ast::Types::Variable(in_var) => {
                        if in_var.name == serch.to_string() {
                            match in_var.node[0].clone() {
                                Ast::Types::Variable(var) => {
                                    let var_name = var.name;
                                    variable_retrun = self.serch_variable(&var_name, vec_function);
                                }

                                Ast::Types::Call(_) => {
                                    let (_, result) = Interpreter::run_judg(&in_var.node[0], self, vec_function);
                                    match result {
                                        Some(somes) => {
                                            variable_retrun = somes;
                                        }

                                        None =>{}
                                    }
                                }

                                _ => {
                                    variable_retrun = in_var.node[0].clone();
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        return variable_retrun;
    }

    //関数も対応
    pub fn variables_allocation(&mut self, serch: Vec<Ast::Types>, vec_function: &mut Function::function) -> Vec<Ast::Types> {
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
                    let serch_result = self.serch_variable(&var.name, vec_function);
                    match serch_result {
                        Ast::Types::Number(mut num) => {
                            num.node = vec;
                            ast_vec.push(Ast::Types::Number(num));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        return ast_vec;
    }
}