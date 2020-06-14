use super::to_c::Compile;
use super::super::ast::asts;

impl Compile {
  pub(crate) fn variable_wirte(&mut self, var:&asts::VariableAST){
    let var_name = &var.name;
    match var.node[0].clone() {
      asts::Types::Strings(strings) => {
        let mut string_var = "".to_string();
        string_var.push_str("char ");
        string_var.push_str(var_name);
        string_var.push_str("[] ");
        string_var.push_str("= \"");
        string_var.push_str(&strings.name);
        string_var.push_str("\";");
        self.write(&string_var);
      }

      asts::Types::Number(num) => {
        let mut num_var = "".to_string();
        num_var.push_str("int ");
        num_var.push_str(var_name);
        num_var.push_str(" = ");
        num_var.push_str(&num.val.to_string());
        num_var.push_str(";");
        self.write(&num_var);
      }

      asts::Types::Call(call) => {
        if call.callee == "stdin" {
          let mut call_var = "".to_string();
          call_var.push_str("char ");
          call_var.push_str(var_name);
          call_var.push_str("[1000];\n");
          call_var.push_str("scanf(\"%s\",");
          call_var.push_str(var_name);
          call_var.push_str(");");
          self.write(&call_var);
        }else {
          
        }
      }
      _ => {}
    }

    self.write("\n");
  }
}