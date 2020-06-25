use super::super::ast::asts;
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
          match sertch_type{
            Some(types) => match types {
              asts::VariableTypes::Bool => {
                values.push_str("%s\\n\", ");
                if change {
                  values.push_str(&format!("atoi({})", &var.name));
                }else{
                  values.push_str(&format!("{}", &var.name));
                }

                values.push_str("? \"true\": \"false\"");
              }

              asts::VariableTypes::Int => {
                values.push_str("%d\\n\", ");
                if change {
                  values.push_str(&format!("atoi({})", &var.name));
                }else{
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
        asts::Types::Function(funs) => match &funs.return_type {
          Some(f) => {
            self.variable.vec_push();

            let mut types = Types::new(&funs.name, &f);
            let addr = self.function.push(&types);
            self.functions(&f, &funs, &mut types);
            self.function.appo_push(addr, &types);

            self.variable.last_remove();
          }

          None => {
            self.variable.vec_push();

            let mut types = Types::new(&funs.name, &asts::VariableTypes::Void);
            let addr = self.function.push(&types);
            self.functions(&asts::VariableTypes::Void, &funs, &mut types);
            self.function.appo_push(addr, &types);

            self.variable.last_remove();
          }
        },
        _ => {}
      }
      index += 1;
    }
  }

  fn functions(&mut self, types: &asts::VariableTypes, fun: &asts::FunctionAST, param: &mut Types) {
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

    self.write(&fun.name);
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
    self.inner_name = Some(fun.name.clone());
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
        err.exit("return number error");
      }
    }

    self.write(";\n");
  }
}
