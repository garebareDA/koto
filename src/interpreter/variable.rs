use super::super::ast::asts;
use super::function;
use super::interpreters;
use super::error;

pub struct Variable {
    variables: Vec<Vec<asts::Types>>,
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

    pub fn push(&mut self, var: asts::Types) {
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
        var: asts::Types,
        vec_function: &mut function::Function,
    ) -> asts::Types {
        let bin_op:char;
        let bin_node:Vec<asts::Types>;

        match var.clone() {
            asts::Types::Binary(bin) => {
                bin_op = bin.op;
                bin_node = bin.node;
            }

            _ => return var,
        }

        if bin_op == '='{
            match bin_node[0] {
                asts::Types::Variable(_) =>{
                    return bin_node[bin_node.len() - 1].clone();
                }

                _ => {}
            }
        }

        if bin_op =='.' {
            //名前空間用の関数処理
            println!("{:?}", bin_node);
        }

        return interpreters::calculation(&var, self, vec_function);
    }

    pub fn serch_variable(
        &mut self,
        serch: &asts::VariableAST,
        vec_function: &mut function::Function,
    ) -> asts::Types {
        let mut variable_retrun = asts::Types::Error(asts::ErrorAST::new("Vairable Error"));
        let var_vec = self.variables.clone();
        let serch = serch.clone();
        let serch_word = &serch.name;

        for vars in var_vec {
            for var in vars {
                let mut in_var = asts::VariableAST::new("");
                match var {
                    asts::Types::Variable(in_vars) => {
                        in_var = in_vars;
                    }

                    _ => {
                        let err = error::Error::new(&var);
                        err.exit("This is not a variable");
                    }
                }

                if &in_var.name == serch_word {
                    match in_var.node[0].clone() {
                        asts::Types::Variable(var) => {
                            variable_retrun = self.serch_variable(&var, vec_function);
                        }

                        asts::Types::Call(_) => {
                            let (_, result) =
                                interpreters::run_judg(&in_var.node[0], self, vec_function);
                            match result {
                                Some(somes) => {
                                    variable_retrun = somes;
                                }

                                None => {
                                    let err = error::Error::new(&in_var.node[0]);
                                    err.exit("vaiable not found");
                                }
                            }
                        }

                        asts::Types::Vector(vector) => match &serch.index {
                            Some(index) => match &index[0] {
                                asts::Types::Number(num) => {
                                    let vector_num = num.val;
                                    let var = &vector.node[vector_num as usize];
                                    variable_retrun = var.clone();
                                }

                                asts::Types::Binary(_) => {
                                    let result = interpreters::calculation(&index[0], self, vec_function);
                                    match result {
                                        asts::Types::Number(num) => {
                                            let vector_num = num.val;
                                            let var = &vector.node[vector_num as usize];
                                            variable_retrun = var.clone();
                                        }
                                        _ => {
                                            let err = error::Error::new(&result);
                                            err.exit("not a Number");
                                        }
                                    }
                                }

                                _ => {
                                    let err = error::Error::new(&index[0]);
                                    err.exit("not a vector");
                                }
                            },
                            None => {
                                let err = error::Error::new(&asts::Types::Variable(serch.clone()));
                                err.exit("not a vector");
                            }
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
        serch: Vec<asts::Types>,
        vec_function: &mut function::Function,
    ) -> Vec<asts::Types> {
        let mut ast_vec: Vec<asts::Types> = Vec::new();
        for node in serch {
            match node.clone() {
                asts::Types::Binary(mut bin) => {
                    if !bin.node.is_empty() {
                        let vec = self.variables_allocation(bin.node.clone(), vec_function);
                        bin.node = vec;
                    }
                    ast_vec.push(asts::Types::Binary(bin));
                }
                asts::Types::Number(mut num) => {
                    if !num.node.is_empty() {
                        let vec = self.variables_allocation(num.node.clone(), vec_function);
                        num.node = vec;
                    }
                    ast_vec.push(asts::Types::Number(num));
                }
                asts::Types::Variable(var) => {
                    let mut vec: Vec<asts::Types> = Vec::new();
                    if !var.node.is_empty() {
                        vec = self.variables_allocation(var.node.clone(), vec_function);
                    }
                    let serch_result = self.serch_variable(&var, vec_function);
                    match serch_result {
                        asts::Types::Number(mut num) => {
                            num.node = vec;
                            ast_vec.push(asts::Types::Number(num));
                        }
                        _ => {
                            let err = error::Error::new(&node);
                            err.exit("variable not found");
                        }
                    }
                }

                asts::Types::Strings(mut string) => {
                    if !string.node.is_empty() {
                        let vec = self.variables_allocation(string.node.clone(), vec_function);
                        string.node = vec;
                    }
                    ast_vec.push(asts::Types::Strings(string));
                }

                asts::Types::Call(call) => {
                    let result = vec_function.function_run(&call, self);
                    match result {
                        Some(result) => {
                            ast_vec.push(result);
                        }

                        None => {
                            let err = error::Error::new(&node);
                            err.exit("not a function");
                        }
                    }
                }
                _ => {
                    let err = error::Error::new(&node);
                    err.exit("not a function");
                }
            }
        }
        return ast_vec;
    }
}
