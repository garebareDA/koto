use super::super::ast::asts;
use super::to_c::Compile;

impl Compile {
  pub(crate) fn call_write(&mut self, call_ast: &asts::CallAST) {
    let callee = &call_ast.callee;
    if callee == "print" {
      self.write("printf(\"");

      let value = &call_ast.argument[0];
      match value {
        asts::Types::Variable(var) => {
          let mut values = "".to_string();
          match self.variable.sertch_type(&var.name) {
            Some(types) => match types {
              asts::VariableTypes::Bool => {
                values.push_str("%s\\n\", ");
                values.push_str(&format!("atoi({})",&var.name));
                values.push_str("? \"true\": \"false\"");
              }

              asts::VariableTypes::Int => {
                values.push_str("%d\\n\", ");
                values.push_str(&format!("atoi({})",&var.name));
              }

              asts::VariableTypes::Strings => {
                values.push_str("%s\\n\", ");
                values.push_str(&var.name);
              }

              _ => {
                //TODO err
              }
            },

            None => {
              //TODO error
            }
          }
          self.write(&values);
        }

        asts::Types::Binary(bin) => {
          self.calcuration(bin, "");
        }

        asts::Types::Strings(strings) => {
          let mut values = "".to_string();
          values.push_str("%s\\n\", \"");
          values.push_str(&strings.name);
          values.push_str("\"");
          self.write(&values);
        }

        _ => {}
      }

      self.write(");\n");
    }
  }

  pub(crate) fn function_write() {
    
  }
}
