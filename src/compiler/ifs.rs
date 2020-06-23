use super::super::ast::asts;
use super::to_c::Compile;

impl Compile {
  pub(crate) fn ifs_write(&mut self, judg: &asts::IfAST) {
    self.variable.vec_push();
    self.function.vec_push();

    match &judg.judge[0] {
      asts::Types::Binary(bin) => {
        self.calcuration(&bin, "tmp");
        self.write(";\nif(atoi(tmp))");
      }

      asts::Types::Variable(vars) => {
        match self.variable.sertch_type(&vars.name) {
          Some(t) => match t {
            asts::VariableTypes::Strings => {
              self.write(&vars.name);
            }

            _ => {
              self.write(&format!("\nif(itoa({})", vars.name));
            }
          },
          _ => {
            //error
          }
        }
      }

      _ => {
        //error
      }
    }

    self.write("{\n");
    self.scope(&judg.node);
    self.write("}\n");
    self.variable.last_remove();
    self.function.last_remove();
  }
}
