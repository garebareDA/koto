use super::super::ast::asts;
use super::to_c::Compile;

impl Compile {
  pub(crate) fn calcuration(&mut self, bin: &asts::BinaryAST) {
    let op = &bin.op.to_string();
    let node = &bin.node[0];
    let in_node = &bin.node[1];
    let mut num_str = "".to_string();

    match node {
      asts::Types::Number(num) => {
        num_str.push_str(&num.val.to_string());
      }

      _ => {}
    }

    num_str.push_str(&format!(" {}", op));
    self.write(&num_str);
    self.calcuration_write(in_node);
  }

  fn calcuration_write(&mut self, node: &asts::Types) {
    match node {
      asts::Types::Number(num) => {
        self.write(&format!(" {}", num.val));

        if num.node.is_empty() {
          return;
        }

        self.calcuration_write(&num.node[0]);
      }

      asts::Types::Binary(bin) => {
        self.write(&format!(" {}", bin.op));

        if bin.node.is_empty() {
          return;
        }

        self.calcuration_write(&bin.node[0]);
      }

      _ => {}
    }
  }
}
