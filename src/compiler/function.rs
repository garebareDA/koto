use super::super::ast::asts;
use super::variable::Types;
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
                values.push_str(&format!("atoi({})", &var.name));
                values.push_str("? \"true\": \"false\"");
              }

              asts::VariableTypes::Int => {
                values.push_str("%d\\n\", ");
                values.push_str(&format!("atoi({})", &var.name));
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

  pub(crate) fn function_write(&mut self, nodes: &Vec<asts::Types>) {
    let mut index = 0;
    let len = nodes.len();
    loop {
      if index >= len {
        break;
      }

      let node = nodes[index].clone();
      match node {
        asts::Types::Function(funs) => {
          match funs.return_type {
            Some(f) => {
              let types = Types::new(&funs.name, &f);
              self.function.push(&types);
            }

            None => {
              let types = Types::new(&funs.name, &asts::VariableTypes::Void);
              self.function.push(&types);
            }
          }
        }
        _ => {}
      }
      index += 1;
    }
  }

  fn functions(&mut self, types:&asts::VariableTypes, fun:asts::FunctionAST) {
    match types {
      asts::VariableTypes::Bool => {
        self.write("int ");
      }

      asts::VariableTypes::Int => {
        self.write("int ");
      }

      asts::VariableTypes::Strings => {
        self.write("char* ");
      }

      asts::VariableTypes::Void => {
        self.write("void ");
      }

      _ => {
        //error
      }
    }

    self.write(&fun.name);
    self.write("(");

    for arg in &fun.argument {
      match arg {
        asts::Types::Number(num) => {
          
        }

        _ => {
          //error
        }
      }
    }
  }
}
