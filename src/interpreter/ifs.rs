use super::super::ast::asts;
use super::interpreters;
use super::function;
use super::variable;

pub struct If {
    pub result: asts::Types,
    pub ifs: Vec<asts::Types>
}

impl If {
    pub fn new(result: &asts::Types, ifs: &Vec<asts::Types>) -> If {
        If{
            result: result.clone(),
            ifs: ifs.clone()
        }
    }

    pub fn if_run(
        self,
        vec_variable: &mut variable::Variable,
        vec_function: &mut function::Function,
    ) -> Option<asts::Types> {
        match self.result {
            asts::Types::Boolean(boolean) => {
                if boolean.boolean {
                    let (_, result) = interpreters::scope(&self.ifs, vec_variable, vec_function);
                    return result;
                }
            }
            _ => {}
        }
        return None;
    }
}