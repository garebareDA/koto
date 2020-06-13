use super::super::ast::asts;
use super::error;
use super::function;
use super::interpreters;

#[derive(Debug)]
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
        var: &asts::VariableAST,
        vec_function: &mut function::Function,
    ) -> Option<asts::Types> {
        let bin_op: char;
        let bin_node: Vec<asts::Types>;
        let mut is_variable: bool = false;

        match var.node[0].clone() {
            asts::Types::Binary(bin) => {
                bin_op = bin.op;
                bin_node = bin.node;
            }

            asts::Types::Variable(inner) => {
                is_variable = true;
                if inner.node.is_empty() {
                    return Some(asts::Types::Variable(inner.clone()));
                }

                match inner.node[0].clone() {
                    asts::Types::Binary(bin) => {
                        let (serch, _) = self.serch_variable(&inner, vec_function);
                        vec_function.push(serch);
                        bin_op = bin.op;
                        bin_node = bin.node;
                    }
                    _ => return None,
                }
            }

            asts::Types::Call(calle) => {
                return vec_function.function_run(&calle, self);
            }

            _ => return Some(var.node[0].clone()),
        }

        if bin_op == '=' {
            match bin_node[0].clone() {
                asts::Types::Variable(_) => {
                    let (_, mutable) = self.serch_variable(var, vec_function);
                    if !mutable {
                        let err = error::Error::new(&asts::Types::Variable(var.clone()));
                        err.exit("variable is mutable");
                    }
                    return Some(bin_node[bin_node.len() - 1].clone());
                }

                _ => {
                    let err = error::Error::new(&bin_node[0]);
                    err.exit("variable error");
                }
            }
        }

        if bin_op == '.' {
            match bin_node[0].clone() {
                asts::Types::Call(fun) => {
                    if is_variable {
                        vec_function.vec_push();
                        let result = vec_function.function_run(&fun, self);
                        vec_function.last_remove();
                        return result;
                    } else {
                        vec_function.vec_push();
                        let (serch, _) = self.serch_variable(&var, vec_function);
                        vec_function.push(serch);
                        let result = vec_function.function_run(&fun, self);
                        vec_function.last_remove();
                        return result;
                    }
                }
                _ => {
                    let err = error::Error::new(&bin_node[0]);
                    err.exit("variable or function error");
                }
            }
        }

        let calcu = interpreters::calculation(&var.node[0].clone(), self, vec_function);
        return Some(calcu);
    }

    pub fn serch_variable(
        &mut self,
        serch: &asts::VariableAST,
        vec_function: &mut function::Function,
    ) -> (Vec<asts::Types>, bool) {
        let mut variable_retrun = Vec::new();
        let mut is_mutable = true;
        let var_vec = self.variables.clone();
        let serch = serch.clone();
        let serch_word = &serch.name;

        for vars in var_vec {
            for var in vars {
                let mut in_var = asts::VariableAST::new("");
                match var {
                    asts::Types::Variable(in_vars) => {
                        in_var = in_vars;
                        is_mutable = in_var.muttable;
                    }

                    _ => {
                        let err = error::Error::new(&var);
                        err.exit("This is not a variable");
                    }
                }

                if &in_var.name == serch_word {
                    match in_var.node[0].clone() {
                        asts::Types::Variable(var) => {
                            let (variable_retruns, _) = self.serch_variable(&var, vec_function);
                            variable_retrun = variable_retruns;
                        }

                        asts::Types::Call(_) => {
                            let (_, result) =
                                interpreters::run_judg(&in_var.node[0], self, vec_function);
                            match result {
                                Some(somes) => {
                                    let mut vec = Vec::new();
                                    vec.push(somes);
                                    variable_retrun = vec;
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
                                    let mut vec = Vec::new();
                                    vec.push(var.clone());
                                    variable_retrun = vec;
                                }

                                asts::Types::Binary(_) => {
                                    let result =
                                        interpreters::calculation(&index[0], self, vec_function);
                                    match result {
                                        asts::Types::Number(num) => {
                                            let vector_num = num.val;
                                            let var = &vector.node[vector_num as usize];
                                            let mut vec = Vec::new();
                                            vec.push(var.clone());
                                            variable_retrun = vec;
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
                            variable_retrun = in_var.node;
                        }
                    }
                }
            }
        }

        if variable_retrun.is_empty() {
            let err = error::Error::new(&asts::Types::Variable(serch.clone()));
            err.exit("variable is empty");
        }

        return (variable_retrun, is_mutable);
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
                asts::Types::Boolean(mut bools) => {
                    if !bools.node.is_empty() {
                        let vec = self.variables_allocation(bools.node.clone(), vec_function);
                        bools.node = vec;
                    }
                    ast_vec.push(asts::Types::Boolean(bools));
                }
                asts::Types::Variable(var) => {
                    let mut vec: Vec<asts::Types> = Vec::new();
                    let mut serch_result: Vec<asts::Types> = Vec::new();
                    let mut is_name_space_function = false;
                    if !var.node.is_empty() {
                        let function_call = self.variable(&var, vec_function);
                        match function_call {
                            Some(call) => {
                                serch_result.push(call);
                                is_name_space_function = true;
                                match var.node[0].clone() {
                                    asts::Types::Binary(bin) => match bin.node[0].clone() {
                                        asts::Types::Call(call) => {
                                            vec = self.variables_allocation(
                                                call.node.clone(),
                                                vec_function,
                                            );
                                        }
                                        _ => {}
                                    },
                                    _ => {}
                                }
                            }
                            None => {}
                        }
                    }

                    if !is_name_space_function {
                        let (serch_results, _) = self.serch_variable(&var, vec_function);
                        serch_result = serch_results;
                        vec = self.variables_allocation(var.node.clone(), vec_function);
                    }

                    match serch_result[0].clone() {
                        asts::Types::Number(mut num) => {
                            num.node = vec;
                            ast_vec.push(asts::Types::Number(num));
                        }
                        asts::Types::Strings(mut strings) => {
                            strings.node = vec;
                            ast_vec.push(asts::Types::Strings(strings));
                        }
                        asts::Types::Boolean(mut bools) => {
                            bools.node = vec;
                            ast_vec.push(asts::Types::Boolean(bools));
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
