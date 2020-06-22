use super::to_c::Compile;
use super::super::ast::asts;

impl Compile {
  pub(crate) fn fors_write(&mut self, fors: &asts::ForAST){
    self.variable.vec_push();

    
  }
}