use super::super::ast::asts;
use super::to_c::Compile;

impl Compile {
  pub(crate) fn function_write(&mut self, call_ast: &asts::CallAST) {
    let callee = &call_ast.callee;
    if callee == "print" {
      self.write("  printf(\"");

      let value = &call_ast.argument[0];
      match value {
        asts::Types::Variable(var) => {
          let mut values = "".to_string();
          //TODO 変数を保存して型を調べる
          values.push_str("%s\\n\", ");
          values.push_str(&var.name);
          self.write(&values);
        }

        asts::Types::Binary(bin) => {
          //TODO
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
}
