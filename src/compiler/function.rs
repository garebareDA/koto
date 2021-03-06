use super::super::ast::asts;
use super::super::interpreter;
use super::super::interpreter::error;
use super::to_c::Compile;
use super::variable::Types;

impl Compile {
  pub(crate) fn call_write(&mut self, call_ast: &asts::CallAST) {
    let callee = &call_ast.callee;
    if callee == "print" {
      self.write("printf(\"");

      let value = &call_ast.argument[0];
      match value {
        asts::Types::Variable(var) => {
          let mut values = "".to_string();
          let (sertch_type, _, change) = self.variable.sertch_type(&var.name);
          match sertch_type {
            Some(types) => match types {
              asts::VariableTypes::Bool => {
                values.push_str("%s\\n\", ");
                if change {
                  values.push_str(&format!("atoi({})", &var.name));
                } else {
                  values.push_str(&format!("{}", &var.name));
                }

                values.push_str("? \"true\": \"false\"");
              }

              asts::VariableTypes::Int => {
                values.push_str("%d\\n\", ");
                if change {
                  values.push_str(&format!("atoi({})", &var.name));
                } else {
                  values.push_str(&format!("{}", &var.name));
                }
              }

              asts::VariableTypes::Strings => {
                values.push_str("%s\\n\", ");
                values.push_str(&var.name);
              }

              _ => {
                let err = error::Error::new(value);
                err.exit("argment error");
              }
            },

            None => {
              let err = error::Error::new(value);
              err.exit("argment error");
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
        asts::Types::Import(imports) => match &imports.path[0] {
          asts::Types::Strings(strings) => {
            self.variable.vec_push();
            let result = interpreter::interpreters::read_file(&strings.name);
            let name: Vec<&str> = strings.name.split('.').collect();
            let name: Vec<&str> = name[name.len() - 2].split('/').collect();
            self.to_import(&name[2]);
            self.function_write(&result.node);
            self.variable.last_remove();
            self.to_import("");
          }
          _ => {}
        },

        asts::Types::Function(funs) => match &funs.return_type {
          Some(f) => {
            self.variable.vec_push();
            if self.import == "" {
              let mut types = Types::new(&funs.name, &f);
              let addr = self.function.push(&types);
              self.functions(&f, &funs, &mut types, &funs.name);
              self.function.appo_push(addr, &types);
            } else {
              let funs_name = &format!("import_{}_{}",  self.import, funs.name);
              let mut types = Types::new(funs_name, &f);
              let addr = self.function.push(&types);
              self.functions(&f, &funs, &mut types, funs_name);
              self.function.appo_push(addr, &types);
            }

            self.variable.last_remove();
          }

          None => {
            self.variable.vec_push();
            if  self.import == "" {
              let mut types = Types::new(&funs.name, &asts::VariableTypes::Void);
              let addr = self.function.push(&types);
              self.functions(&asts::VariableTypes::Void, &funs, &mut types, &funs.name);
              self.function.appo_push(addr, &types);
            } else {
              let funs_name = &format!("import_{}_{}",  self.import, funs.name);
              let mut types = Types::new(funs_name, &asts::VariableTypes::Void);
              let addr = self.function.push(&types);
              self.functions(&asts::VariableTypes::Void, &funs, &mut types, funs_name);
              self.function.appo_push(addr, &types);
            }
            self.variable.last_remove();
          }
        },
        _ => {}
      }
      index += 1;
    }
  }

  fn functions(
    &mut self,
    types: &asts::VariableTypes,
    fun: &asts::FunctionAST,
    param: &mut Types,
    fun_name: &str,
  ) {
    match types {
      asts::VariableTypes::Bool => {
        self.write("int ");
      }

      asts::VariableTypes::Int => {
        self.write("int ");
      }

      asts::VariableTypes::Strings => {
        self.write("char *");
      }

      asts::VariableTypes::Void => {
        self.write("void ");
      }

      _ => {
        let err = error::Error::new(&asts::Types::Function(fun.clone()));
        err.exit("function error");
      }
    }

    self.write(fun_name);
    self.write("(");

    for (i, arg) in fun.argument.iter().enumerate() {
      match arg {
        asts::Types::Variable(vars) => {
          match &vars.types {
            Some(t) => match t {
              asts::VariableTypes::Strings => {
                let types = Types::new(&vars.name, &asts::VariableTypes::Strings);
                self.variable.push(&types);
                self.write("char *");
                param.array_push(&asts::VariableTypes::Strings);
              }

              asts::VariableTypes::Bool => {
                let types = Types::new(&vars.name, &asts::VariableTypes::Bool);
                self.variable.push(&types);
                self.write("int ");
                param.array_push(&asts::VariableTypes::Bool);
              }

              asts::VariableTypes::Int => {
                let types = Types::new(&vars.name, &asts::VariableTypes::Int);
                self.variable.push(&types);
                self.write("int ");
                param.array_push(&asts::VariableTypes::Int);
              }

              _ => {
                let err = error::Error::new(arg);
                err.exit("param error");
              }
            },

            None => {
              let err = error::Error::new(arg);
              err.exit("param error");
            }
          }
          self.write(&vars.name);
        }
        _ => {
          let err = error::Error::new(arg);
          err.exit("param error");
        }
      }

      if i != fun.argument.len() - 1 {
        self.write(",");
      }
    }
    self.write(")\n");
    self.write("{\n");
    self.inner_name = Some(fun_name.to_string());
    self.scope(&fun.node);
    self.write("}\n");
  }

  pub(crate) fn return_write(&mut self, rets: &asts::RetrunAST, function_name: &str) {
    self.write("return ");

    if rets.node.is_empty() {
      self.write(";");
      return;
    }

    let types = self.function.sertch_type(function_name).0;
    match &rets.node[0] {
      asts::Types::Binary(bin) => {
        self.calcuration(&bin, "tmp");
        match &types {
          Some(t) => match t {
            asts::VariableTypes::Strings => {
              self.write("tmp");
            }

            asts::VariableTypes::Bool => {
              self.write("atoi(tmp)");
            }

            asts::VariableTypes::Int => {
              self.write("atoi(tmp)");
            }

            _ => {
              let err = error::Error::new(&rets.node[0]);
              err.exit("return binary error");
            }
          },

          None => {
            let err = error::Error::new(&rets.node[0]);
            err.exit("return binary error");
          }
        }
      }

      asts::Types::Boolean(bools) => match &types {
        Some(t) => match t {
          asts::VariableTypes::Bool => {
            if bools.boolean == true {
              self.write("1");
            } else {
              self.write("0");
            }
          }
          _ => {
            let err = error::Error::new(&rets.node[0]);
            err.exit("return bool error");
          }
        },
        None => {
          let err = error::Error::new(&rets.node[0]);
          err.exit("return bool error");
        }
      },

      asts::Types::Strings(strings) => match &types {
        Some(t) => match t {
          asts::VariableTypes::Strings => {
            self.write(&strings.name);
          }
          _ => {
            let err = error::Error::new(&rets.node[0]);
            err.exit("return string error");
          }
        },
        None => {
          let err = error::Error::new(&rets.node[0]);
          err.exit("return string error");
        }
      },

      asts::Types::Variable(vars) => {
        let serch_types = self.variable.sertch_type(&vars.name).0;
        match &types {
          Some(t) => match t {
            asts::VariableTypes::Bool => match serch_types.unwrap() {
              asts::VariableTypes::Bool => {
                self.write(&vars.name);
              }

              _ => {
                let err = error::Error::new(&rets.node[0]);
                err.exit("return variable error");
              }
            },

            asts::VariableTypes::Int => match serch_types.unwrap() {
              asts::VariableTypes::Int => {
                self.write(&vars.name);
              }

              _ => {
                let err = error::Error::new(&rets.node[0]);
                err.exit("return variable error");
              }
            },

            asts::VariableTypes::Strings => match serch_types.unwrap() {
              asts::VariableTypes::Strings => {
                self.write(&vars.name);
              }

              _ => {
                let err = error::Error::new(&rets.node[0]);
                err.exit("return variable error");
              }
            },

            _ => {
              let err = error::Error::new(&rets.node[0]);
              err.exit("return variable error");
            }
          },

          None => {
            let err = error::Error::new(&rets.node[0]);
            err.exit("return variable error");
          }
        }
      }

      asts::Types::Number(num) => match &types {
        Some(t) => match t {
          asts::VariableTypes::Int => {
            self.write(&num.val.to_string());
          }
          _ => {
            let err = error::Error::new(&rets.node[0]);
            err.exit("return number error");
          }
        },
        None => {
          let err = error::Error::new(&rets.node[0]);
          err.exit("return number error");
        }
      },

      _ => {
        let err = error::Error::new(&rets.node[0]);
        err.exit("return error");
      }
    }

    self.write(";\n");
  }

  pub(crate) fn argment_write(&mut self, argment: &Vec<asts::Types>, callee: &str) {
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

  pub(crate) fn var_call_write(
    &mut self,
    call: &asts::CallAST,
    var_name: &str,
    call_name: &str,
    var: &asts::VariableAST,
  ) {
    let mut types:Option<asts::VariableTypes> = None;
    if self.import != ""{
      let call_name = &format!("import_{}_{}", self.import, call_name);
      types = self.function.sertch_type(call_name).0;
    }else {
      types = self.function.sertch_type(call_name).0;
    }
    match types {
      Some(t) => {
        let mut call_var = self.type_write(&t, &var_name, &asts::Types::Call(call.clone()));
        call_var.push_str(call_name);
        call_var.push_str("(");
        self.write(&call_var);
        self.argment_write(&call.argument, call_name);
        self.write(");");
      }
      None => {
        let err = error::Error::new(&var.node[0].clone());
        err.exit("error function or variable");
      }
    }
  }
}
