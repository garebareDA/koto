use super::super::ast::asts;
use super::super::interpreter::error;
use super::to_c::Compile;

#[derive(Debug, Clone)]
pub struct Types {
  name: String,
  types: asts::VariableTypes,
  change:bool,
  array: Vec<asts::VariableTypes>,
}

impl Types {
  pub fn new(name: &str, types: &asts::VariableTypes) -> Types {
    Types {
      name: name.to_string(),
      types: types.clone(),
      change:false,
      array: Vec::new(),
    }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_type(&self) -> &asts::VariableTypes {
    &self.types
  }

  pub fn array_push(&mut self, param: &asts::VariableTypes) {
    self.array.push(param.clone());
  }

  pub fn change(&mut self) {
    if self.change {
      self.change = false;
    }else{
      self.change = true;
    }
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

  pub fn push(&mut self, var: &Types) -> usize{
    self.variables[self.inner].push(var.clone());
    let address = self.variables.len() - 1;
    return address;
  }

  pub fn appo_push(&mut self,vec:usize,var: &Types) {
    self.variables[self.inner][vec] = var.clone();
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

  pub fn get_len(&self) -> usize {
    self.variables.len() - 1
  }

  pub fn sertch_type(&self, name: &str) -> (Option<asts::VariableTypes>, Vec<asts::VariableTypes>, bool) {
    let mut vars_vec = self.variables.clone();
    vars_vec.reverse();
    for vars in vars_vec {
      for var in vars {
        if var.name == name {
          return (Some(var.types), var.array, var.change);
        }
      }
    }
    return (None, Vec::new(), false);
  }
}

impl Compile {
  pub(crate) fn variable_wirte(&mut self, var: &asts::VariableAST) {
    let var_name = &var.name;
    match var.node[0].clone() {
      asts::Types::Binary(bin) => {
        let types = self.calcuration(&bin, var_name);
        self.write(";");

        let mut types = Types::new(var_name, &types);
        types.change();
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
          let types = self.function.sertch_type(&call.callee).0;
          match types {
            Some(t) => {
              let mut call_var = "".to_string();
              match t {
                asts::VariableTypes::Strings => {
                  let types = Types::new(var_name, &asts::VariableTypes::Strings);
                  self.variable.push(&types);

                  call_var.push_str("char ");
                  call_var.push_str(var_name);
                  call_var.push_str("[] =");
                }

                asts::VariableTypes::Bool => {
                  let types = Types::new(var_name, &asts::VariableTypes::Bool);
                  self.variable.push(&types);

                  call_var.push_str("int ");
                  call_var.push_str(var_name);
                  call_var.push_str("=");
                }

                asts::VariableTypes::Int => {
                  let types = Types::new(var_name, &asts::VariableTypes::Int);
                  self.variable.push(&types);

                  call_var.push_str("int ");
                  call_var.push_str(var_name);
                  call_var.push_str(" = ");
                }
                _ => {
                  let err = error::Error::new(&var.node[0].clone());
                  err.exit("error variable");
                }
              }
              call_var.push_str(&call.callee);
              call_var.push_str("(");
              self.write(&call_var);
              self.argment_write(call.argument, &call.callee);
              self.write(");");
            }

            None => {
              let err = error::Error::new(&var.node[0].clone());
              err.exit("error variable");
            }
          }
        }
      }
      _ => {
        let err = error::Error::new(&var.node[0].clone());
        err.exit("error variable");
      }
    }
  }

  fn argment_write(&mut self, argment: Vec<asts::Types>, callee: &str) {
    if argment.len() == 0 {
      return;
    }

    let call_types = self.function.sertch_type(callee);
    let type_array = call_types.1;

    for (i, arg) in argment.iter().enumerate() {
      let _param_types = &type_array[i];
      match arg {
        asts::Types::Variable(var) => {
          match self.variable.sertch_type(&var.name).0 {
            Some(t) => match t {
              _param_types => {}

              _ => {
                let err = error::Error::new(arg);
                err.exit("argment type error");
              }
            },

            None => {
              let err = error::Error::new(arg);
              err.exit("argment type error");
            }
          }
          self.write(&var.name);
        }

        asts::Types::Strings(strings) => {
          self.write(&format!("\"{}\"", strings.name));
        }

        asts::Types::Number(num) => {
          self.write(&num.val.to_string());
        }

        asts::Types::Boolean(bools) => {
          if bools.boolean {
            self.write("1");
          } else {
            self.write("0");
          }
        }

        asts::Types::Call(call) => {
          self.call_write(&call);
          self.write(";\n");
        }

        _ => {
          let err = error::Error::new(arg);
          err.exit("argment error");
        }
      }

      if i != argment.len() - 1 {
        self.write(",");
      }
    }
  }
}
