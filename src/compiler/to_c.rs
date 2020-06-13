use super::super::ast::asts;

pub fn compile(root: asts::ExprAST) {
  let mut index = 0;
  let len = root.node.len();
  loop {
    if index >= len {
      break;
    }

    let node = &root.node[index];
    judge(node);
    index += 1;
  }
}

fn judge(node:&asts::Types) {
  match node {
    asts::Types::Boolean(bools) => {

    }

    _ => {
      return;
    }
  }
}
