use super::super::ast::asts;
use super::to_c::Compile;

#[derive(Debug, Clone)]
pub struct Types {
  name: String,
  types: asts::VariableTypes,
}

impl Types {
  pub fn new(name: &str, types: &asts::VariableTypes) -> Types {
    Types {
      name: name.to_string(),
      types: types.clone(),
    }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_type(&self) -> &asts::VariableTypes {
    &self.types
  }
}

#[derive(Debug, Clone)]
pub struct Vriables {
  variables: Vec<Vec<Types>>,
  inner: usize,
}

impl Vriables {
  pub fn new() -> Vriables {
    let mut var = Vriables {
      variables: Vec::new(),
      inner: 0,
    };

    var.variables.push(Vec::new());
    return var;
  }

  pub fn last_remove(&mut self) {
    self.variables.remove(self.variables.len() - 1);
    self.out_var();
  }

  pub fn vec_push(&mut self) {
    self.variables.push(Vec::new());
    self.in_var();
  }

  pub fn push(&mut self, var: &Types) {
    self.variables[self.inner].push(var.clone());
  }

  fn in_var(&mut self) {
    self.inner += 1;
  }

  fn out_var(&mut self) {
    if self.inner == 0 {
      return;
    }
    self.inner -= 1;
  }

  pub fn sertch_type(&self, name:&str) -> Option<asts::VariableTypes> {
    let mut vars_vec = self.variables.clone();
    vars_vec.reverse();
    for vars in vars_vec {
      for var in vars {
        if var.name == name {
          return Some(var.types);
        }
      }
    }
    return None;
  }
}

impl Compile {
  pub(crate) fn variable_wirte(&mut self, var: &asts::VariableAST) {
    let var_name = &var.name;
    match var.node[0].clone() {
      asts::Types::Strings(strings) => {
        let types = Types::new(var_name, &asts::VariableTypes::Strings);
        self.variable.push(&types);

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
        let types = Types::new(var_name, &asts::VariableTypes::Int);
        self.variable.push(&types);

        let mut num_var = "".to_string();
        num_var.push_str("int ");
        num_var.push_str(var_name);
        num_var.push_str(" = ");
        num_var.push_str(&num.val.to_string());
        num_var.push_str(";");
        self.write(&num_var);
      }

      asts::Types::Boolean(bools) => {
        let types = Types::new(var_name, &asts::VariableTypes::Bool);
        self.variable.push(&types);

        let mut num_var = "".to_string();
        num_var.push_str("int ");
        num_var.push_str(var_name);
        num_var.push_str(" = ");
        if bools.boolean {
          num_var.push_str("1");
        } else {
          num_var.push_str("0");
        }
        num_var.push_str(";");
        self.write(&num_var);
      }

      asts::Types::Call(call) => {
        if call.callee == "stdin" {
          let types = Types::new(var_name, &asts::VariableTypes::Strings);
          self.variable.push(&types);

          let mut call_var = "".to_string();
          call_var.push_str("char ");
          call_var.push_str(var_name);
          call_var.push_str("[1000];\n");
          call_var.push_str("scanf(\"%s\",");
          call_var.push_str(var_name);
          call_var.push_str(");");
          self.write(&call_var);
        } else {
          /*
          TODO function
          関数の返り値の型を調べる
          それをvariablesにpush
          */
        }
      }
      _ => {}
    }

    self.write("\n");
  }
}
