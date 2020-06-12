use super::super::ast::asts;

pub fn compile(root: asts::ExprAST) {
  let mut index = 0;
  let len = root.node.len();
  loop {
    if index >= len {
      break;
    }

    let node = &root.node[index];
    println!("{:?}", node);
    index += 1;
  }
}
