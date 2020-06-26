use super::super::ast::asts;
use super::super::interpreter::error;
use super::to_c::Compile;

#[derive(Debug, Clone)]
pub struct Types {
  name: String,
  types: asts::VariableTypes,
  change: bool,
  array: Vec<asts::VariableTypes>,
}

impl Types {
  pub fn new(name: &str, types: &asts::VariableTypes) -> Types {
    Types {
      name: name.to_string(),
      types: types.clone(),
      change: false,
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
    } else {
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

  pub fn push(&mut self, var: &Types) -> usize {
    self.variables[self.inner].push(var.clone());
    let address = self.variables[self.inner].len() - 1;
    return address;
  }

  pub fn appo_push(&mut self, vec: usize, var: &Types) {
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

  pub fn sertch_type(
    &self,
    name: &str,
  ) -> (Option<asts::VariableTypes>, Vec<asts::VariableTypes>, bool) {
    let mut vars_vec = self.variables.clone();
    vars_vec.reverse();
    for vars in vars_vec {
      if vars.is_empty() {
        continue;
      }
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
    match &var.node[0] {
      asts::Types::Variable(var) => {
        if var.node.is_empty() {
          let serch_types = self.variable.sertch_type(&var.name).0;
          match serch_types {
            Some(t) => {
              let var_str = self.type_write(&t, var_name, &asts::Types::Variable(var.clone()));
              self.write(&var_str);
              self.write(&var.name);
              self.write(";");
            }

            None => {
              let err = error::Error::new(&var.node[0]);
              err.exit("variable error");
            }
          }
        } else {
          match &var.node[0] {
            asts::Types::Binary(bin) => {
              if bin.op == '.' {
                match &bin.node[0] {
                  asts::Types::Call(call) => {
                    let call_name = &format!("import_{}_{}", var.name, call.callee);
                    self.var_call_write(call, var_name, call_name, var);
                  }

                  _ => {}
                }
              } else {
                let err = error::Error::new(&var.node[0]);
                err.exit("variable error");
              }
            }

            _ => {}
          }
          return;
        }

        match &var.index {
          Some(i) => {
            let sertch_type = self.variable.sertch_type(&var.name).0;
            match sertch_type {
              Some(t) => {
                let mut var_tmp =
                  self.type_write(&t, &var_name, &asts::Types::Variable(var.clone()));
                var_tmp.push_str(&var.name);
                var_tmp.push_str("[");
                match &i[0] {
                  asts::Types::Number(num) => {
                    var_tmp.push_str(&num.val.to_string());
                  }

                  _ => {
                    let err = error::Error::new(&var.node[0]);
                    err.exit("variable error");
                  }
                }
                self.write(&var_tmp);
                self.write("];");
              }

              None => {
                let err = error::Error::new(&var.node[0]);
                err.exit("variable error");
              }
            }
            return;
          }

          None => {}
        }
      }
      asts::Types::Binary(bin) => {
        //変数の再代入
        if bin.op == '=' {
          self.resubstitution(var_name, &bin, var);
          return;
        }

        let types_cal = self.calcuration(&bin, var_name);
        self.write(";");
        let mut types = Types::new(var_name, &types_cal);
        match types_cal {
          asts::VariableTypes::Strings => {
            types.change();
          }
          _ => {}
        }

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
          //callの書き込み
          self.var_call_write(call, var_name, &call.callee, var);
        }
      }

      asts::Types::Vector(vecs) => match &vecs.node[0] {
        asts::Types::Strings(_) => {
          self.write(&format!("char {}[][{}] = ", var_name, vecs.node.len()));
          let types = Types::new(var_name, &asts::VariableTypes::Strings);
          self.variable.push(&types);
          self.array_write(&vecs.node, &asts::VariableTypes::Strings);
          self.write(";");
        }

        asts::Types::Number(_) => {
          self.write(&format!("int {}[{}] = ", var_name, vecs.node.len()));
          let types = Types::new(var_name, &asts::VariableTypes::Int);
          self.variable.push(&types);
          self.array_write(&vecs.node, &asts::VariableTypes::Int);
          self.write(";");
        }

        asts::Types::Boolean(_) => {
          self.write(&format!("int {}[{}] = ", var_name, vecs.node.len()));
          let types = Types::new(var_name, &asts::VariableTypes::Bool);
          self.variable.push(&types);
          self.array_write(&vecs.node, &asts::VariableTypes::Bool);
          self.write(";");
        }

        _ => {
          let err = error::Error::new(&vecs.node[0]);
          err.exit("Vecter error");
        }
      },

      _ => {
        let err = error::Error::new(&var.node[0].clone());
        err.exit("error variable");
      }
    }
  }

  fn array_write(&mut self, arry: &Vec<asts::Types>, types: &asts::VariableTypes) {
    self.write("{");
    for (i, arr) in arry.iter().enumerate() {
      match arr {
        asts::Types::Strings(strings) => match types {
          asts::VariableTypes::Strings => {
            self.write(&format!("\"{}\"", strings.name));
          }

          _ => {
            let err = error::Error::new(arr);
            err.exit("type error");
          }
        },

        asts::Types::Number(num) => match types {
          asts::VariableTypes::Int => {
            self.write(&num.val.to_string());
          }

          _ => {
            let err = error::Error::new(arr);
            err.exit("type error");
          }
        },

        asts::Types::Boolean(bools) => match types {
          asts::VariableTypes::Int => {
            if bools.boolean {
              self.write("1");
            } else {
              self.write("0");
            }
          }

          _ => {
            let err = error::Error::new(arr);
            err.exit("type error");
          }
        },
        _ => {
          let err = error::Error::new(arr);
          err.exit("arry error");
        }
      }

      if i != arry.len() {
        self.write(",");
      }
    }
    self.write("}");
  }

  pub(crate) fn type_write(
    &mut self,
    t: &asts::VariableTypes,
    var_name: &str,
    node: &asts::Types,
  ) -> String {
    let mut types_str = "".to_string();
    match t {
      asts::VariableTypes::Strings => {
        let types = Types::new(var_name, &asts::VariableTypes::Strings);
        self.variable.push(&types);

        types_str.push_str("char ");
        types_str.push_str(var_name);
        types_str.push_str("[] =");
      }

      asts::VariableTypes::Bool => {
        let types = Types::new(var_name, &asts::VariableTypes::Bool);
        self.variable.push(&types);

        types_str.push_str("int ");
        types_str.push_str(var_name);
        types_str.push_str("=");
      }

      asts::VariableTypes::Int => {
        let types = Types::new(var_name, &asts::VariableTypes::Int);
        self.variable.push(&types);

        types_str.push_str("int ");
        types_str.push_str(var_name);
        types_str.push_str(" = ");
      }
      _ => {
        let err = error::Error::new(&node);
        err.exit("error variable");
      }
    }

    return types_str;
  }

  fn resubstitution(&mut self, var_name: &str, bin: &asts::BinaryAST, var: &asts::VariableAST) {
    match &bin.node[1] {
      asts::Types::Variable(vars) => {
        let types = self.variable.sertch_type(&vars.name).0;
        let types_var = self.variable.sertch_type(&var_name).0;
        match types {
          Some(t) => match types_var {
            Some(_ts) => match t {
              _ts => {
                self.write(&format!("{} = {}", var_name, vars.name));
                self.write(";");
              }
            },

            None => {
              let err = error::Error::new(&var.node[0]);
              err.exit("variable error");
            }
          },

          None => {
            let err = error::Error::new(&var.node[0]);
            err.exit("variable error");
          }
        }
      }
      _ => {
        let err = error::Error::new(&var.node[0]);
        err.exit("variable error");
      }
    };
  }

  fn import_function_write(&mut self) {}
}
