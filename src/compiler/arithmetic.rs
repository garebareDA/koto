use super::super::ast::asts;
use super::to_c::Compile;

struct Format{
  formats:String,
  strings:String,
}

impl Format {
  pub fn new() -> Format {
    Format{
      formats:String::new(),
      strings:String::new(),
    }
  }
}

impl Compile {
  pub(crate) fn calcuration(&mut self, bin: &asts::BinaryAST, var_name:&str) {
    self.write(&format!("char {}[1000] = \"\\0\";\n", var_name));
    let mut formats = Format::new();
    let op = &bin.op.to_string();
    let node = &bin.node[0];
    let in_node = &bin.node[1];
    let mut num_str = format!("snprintf({}, 1000, ", var_name);

    match node {
      asts::Types::Number(num) => {
        formats.formats.push_str("%d");
        formats.strings.push_str(&num.val.to_string());
      }

      _ => {}
    }

    formats.strings.push_str(&format!(" {}", op));
    self.write(&num_str);
    self.calcuration_write(in_node);
  }

  fn calcuration_write(&mut self, node: &asts::Types) -> Option<asts::VariableTypes> {
    match node {
      asts::Types::Strings(strings) => {
        self.write(&format!(" \"{}\"", strings.name));
        if strings.node.is_empty() {
          return Some(asts::VariableTypes::Strings);
        }

        self.calcuration_write(&strings.node[0]);
        return Some(asts::VariableTypes::Strings);
      }

      asts::Types::Number(num) => {
        self.write(&format!(" {}", num.val));

        if num.node.is_empty() {
          return Some(asts::VariableTypes::Int);
        }

        self.calcuration_write(&num.node[0]);
        return Some(asts::VariableTypes::Int);
      }

      asts::Types::Binary(bin) => {
        self.write(&format!(" {}", bin.op));

        if bin.node.is_empty() {
          return Some(asts::VariableTypes::Binary);
        }

        self.calcuration_write(&bin.node[0]);
        return Some(asts::VariableTypes::Binary);
      }

      _ => {
        None
      }
    }
  }
}
