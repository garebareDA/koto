use super::to_c::Compile;
use super::super::ast::asts;

impl Compile {
  pub(crate) fn fors_write(&mut self, fors: &asts::ForAST){
    self.variable.vec_push();

    let variant = &fors.init_var[0];
    let judge = &fors.init_var[1];
    let loop_for = &fors.init_var[2];

    self.write("for(");

    match variant {
      asts::Types::Variable(vars) => {
        self.variable_wirte(vars);
      }

      _ => {
        //error
      }
    }

    match judge {


      _ => {
        //error
      }
    }
  }

  fn fors_juge() {

  }
}