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

  pub fn sertch_type(&self, name: &str) -> Option<asts::VariableTypes> {
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
      asts::Types::Binary(bin) => {
        let types = self.calcuration(&bin, var_name);
        self.write(";");

        let types = Types::new(var_name, &types);
        self.variable.push(&types);
      }

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
          match self.function.sertch_type(&call.callee) {
            Some(t) => {
              let mut call_var = "".to_string();
              match t {
                asts::VariableTypes::Strings => {
                  let types = Types::new(var_name,  &asts::VariableTypes::Strings);
                  self.variable.push(&types);

                  call_var.push_str("char ");
                  call_var.push_str(var_name);
                  call_var.push_str("[] =");
                }

                asts::VariableTypes::Bool => {
                  let types = Types::new(var_name,  &asts::VariableTypes::Int);
                  self.variable.push(&types);

                  call_var.push_str("int ");
                  call_var.push_str(var_name);
                  call_var.push_str("=");
                }

                asts::VariableTypes::Int => {
                  let types = Types::new(var_name,  &asts::VariableTypes::Int);
                  self.variable.push(&types);

                  call_var.push_str("int ");
                  call_var.push_str(var_name);
                  call_var.push_str("=");
                }
                _ => {
                  //error
                }
              }
              call_var.push_str(&call.callee);
              call_var.push_str("(");
              self.write(&call_var);
              self.argment_write(call.argument);
              self.write(");");
            }

            None => {
              //error
            }
          }
        }
      }
      _ => {
        //error
      }
    }
  }

  fn argment_write(&mut self, argment:Vec<asts::Types>) {
    for (i,arg) in argment.iter().enumerate(){
      match arg {
        asts::Types::Variable(var) => {
          self.write(&var.name);
        }

        asts::Types::Strings(strings) => {
          self.write(&format!("\"{}\"", strings.name));
        }

        asts::Types::Number(num) => {
          self.write(&num.val.to_string());
        }

        asts::Types::Boolean(bools) => {
          if bools.boolean{
            self.write("1");
          }else{
            self.write("0");
          }
        }

        asts::Types::Call(call) => {
          self.call_write(&call);
          self.write(";\n");
        }

        _ => {
          //error
        }
      }
      if i != argment.len() - 1{
        self.write(",");
      }

    }
  }
}
