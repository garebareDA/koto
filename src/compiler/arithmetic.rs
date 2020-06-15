use super::to_c::Compile;
use super::super::ast::asts;

impl Compile {
  pub(crate) fn calcuration(&mut self, bin: &asts::BinaryAST) {
    let op = &bin.op.to_string();
    let node = &bin.node[0];
    let in_node = &bin.node[1];
    //とりあえず先に四則演算の書き込みをする

    if op == "+" {
      match node {
        asts::Types::Strings(strings) => {

        }

        asts::Types::Boolean(bools) => {

        }

        asts::Types::Number(num) => {
          self.write(&num.val.to_string());
          self.write(" + ");
        }

        _ => {
          //error処理
        }
      }
    }
  }

  fn calcuration_write(&mut self) {

  }
}