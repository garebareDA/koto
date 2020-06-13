use super::to_c::Compile;
use super::super::ast::asts;

impl Compile{
  pub(crate) fn function_write(&mut self, call_ast: &asts::CallAST ) {
    let callee = &call_ast.callee;
    if callee == "print" {
      self.write("  printf(\"");

      let value = &call_ast.argument[0];
      match value {
        asts::Types::Variable(var) => {
          //TODO
        }

        asts::Types::Binary(bin) => {
          //TODO
        }

        _ => {
          self.printf_var(value);
        }
      }

      self.write(");\n");
    }
  }

  fn printf_var(&mut self, var_result: &asts::Types) {
    match var_result {
      asts::Types::Strings(value) => {
        self.write("%s\\n\", \"");
        self.write(&value.name);
        self.write("\"");
      }

      _ => {

      }
    }
  }
}