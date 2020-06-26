use super::super::ast::asts;
use super::super::interpreter::error;
use super::to_c::Compile;

impl Compile {
  pub(crate) fn ifs_write(&mut self, judg: &asts::IfAST) {
    self.variable.vec_push();
    self.function.vec_push();

    match &judg.judge[0] {
      asts::Types::Binary(bin) => {
        let tmp = &format!("tpm{}", self.tmp);
        match self.calcuration(&bin, tmp){
          asts::VariableTypes::Strings => {
            self.write(&format!(";\nif(atoi({}))", tmp));
          }
          _ => {
            self.write(&format!(";\nif({})", tmp));
          }
        }
        self.tmp += 1;
      }

      asts::Types::Variable(vars) => {
        let (sertch_types, _, change) = self.variable.sertch_type(&vars.name);
        match sertch_types {
          Some(t) => match t {
            asts::VariableTypes::Strings => {
              self.write(&vars.name);
            }

            _ => {
              if change {
                self.write(&format!("\nif(itoa({})", vars.name));
              } else {
                self.write(&format!("\nif({})", vars.name));
              }
            }
          },
          _ => {
            let err = error::Error::new(&judg.judge[0]);
            err.exit("if error");
          }
        }
      }

      _ => {
        let err = error::Error::new(&judg.judge[0]);
        err.exit("if error");
      }
    }

    self.write("{\n");
    self.scope(&judg.node);
    self.write("}\n");
    self.variable.last_remove();
    self.function.last_remove();
  }
}
