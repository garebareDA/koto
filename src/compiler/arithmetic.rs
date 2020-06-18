use super::super::ast::asts;
use super::to_c::Compile;

#[derive(Debug, Clone)]
struct Format {
  formats: String,
  strings: String,
}

impl Format {
  pub fn new() -> Format {
    Format {
      formats: String::new(),
      strings: String::new(),
    }
  }
}

impl Compile {
  pub(crate) fn calcuration(&mut self, bin: &asts::BinaryAST, var_name: &str) {
    let mut formats = Format::new();
    let op = &bin.op.to_string();
    let node = &bin.node[0];
    let in_node = &bin.node[1];

    match node {
      asts::Types::Number(num) => {
        formats.formats.push_str("%d");
        formats.strings.push_str(&num.val.to_string());
        formats.strings.push_str(&op);
        self.calcuration_write(in_node, &mut formats, &asts::VariableTypes::Int);
      }

      asts::Types::Strings(strings) => {
        formats.formats.push_str("%s");
        formats.strings.push_str(&format!("\"{}\"", &strings.name));
        if op == "+" {
          formats.strings.push_str(",");
        }else{
          //error
        }

        self.calcuration_write(in_node, &mut formats, &asts::VariableTypes::Strings);
      }

      _ => {}
    }
    formats.strings.remove(formats.strings.len() - 1);
    let formats_len = formats.strings.len();
    self.write(&format!("char {}[{}] = \"\\0\";\n", var_name, formats_len));
    self.write(&format!("snprintf({}, {}, ", var_name, formats_len));
    self.write(&format!("\"{}\",", &formats.formats));
    self.write(&formats.strings);
    self.write(")");
    println!("{:?}", formats);
  }

  fn calcuration_write(
    &mut self,
    node: &asts::Types,
    foramts: &mut Format,
    types: &asts::VariableTypes,
  ) {
    match node {
      asts::Types::Strings(strings) => {
        foramts.formats.push_str("%s");
        foramts.strings.remove(foramts.strings.len() - 1);
        foramts.strings.push_str(&format!(",\"{}\"", &strings.name));

        if strings.node.is_empty() {
          return;
        }

        self.calcuration_write(&strings.node[0], foramts, &asts::VariableTypes::Strings);
        return;
      }

      asts::Types::Number(num) => {
        foramts.strings.push_str(&num.val.to_string());

        match types {
          asts::VariableTypes::Strings => {
            foramts.formats.push_str("%d");
          }
          _ => {}
        }

        if num.node.is_empty() {
          return;
        }

        self.calcuration_write(&num.node[0], foramts, &asts::VariableTypes::Int);
        return;
      }

      asts::Types::Binary(bin) => {
        match types {
          asts::VariableTypes::Strings => {
            if bin.op == '+' {
              foramts.strings.push_str(",");
            }else {
              //error
            }
          },
          asts::VariableTypes::Int => {
            foramts.strings.push_str(&bin.op.to_string());
          },
          _ => {
            //error
          }
        }
        if bin.node.is_empty() {
          return;
        }

        self.calcuration_write(&bin.node[0], foramts, types);
        return;
      }

      asts::Types::Variable(vars) => {
        match self.variable.sertch_type(&vars.name){
          Some(t) => {
            match t {
              asts::VariableTypes::Strings => {
                foramts.formats.push_str("%s");
              },
              asts::VariableTypes::Int => {
                foramts.formats.push_str("%d");
              },

              _ => {
                //error
              }
            }
          }
          None => {
            //error
          }
        }

        foramts.strings.remove(foramts.strings.len() - 1);
        foramts.strings.push_str(&format!(",{},", vars.name));

        if vars.node.is_empty() {
          return;
        }

        self.calcuration_write(&vars.node[0], foramts, types);
        return;
      }

      _ => {},
    }
  }
}
