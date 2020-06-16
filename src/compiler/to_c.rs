use super::super::ast::asts;
use super::variable;
use std::fs;
use std::io::Write;

//変数の関数の方を調べるために配列を用意する

pub struct Compile {
  pub file: std::fs::File,
  pub variable: variable::Vriables
}

impl Compile {
  pub fn new() -> Compile {
    fs::create_dir_all("./build").expect("dir create failed");
    let file = fs::File::create("./build/build.c").expect("create failed");
    let vairables = variable::Vriables::new();
    Compile {
      file: file,
      variable: vairables,
    }
  }

  pub fn write(&mut self, string: &str) {
    self
      .file
      .write_all(string.as_bytes())
      .expect("write failed");
  }

  pub fn compile(&mut self, root: asts::ExprAST) {
    self.write("#include <stdio.h>\n");
    self.write("#include <stdlib.h>\n");
    self.write("int main() {\n");
    let mut index = 0;
    let len = root.node.len();
    loop {
      if index >= len {
        break;
      }

      let node = &root.node[index];
      self.judge(node);
      index += 1;
    }
    self.write("  return 0;\n");
    self.write("}");
  }

  fn judge(&mut self, node: &asts::Types) {
    match node {
      asts::Types::Variable(var) => {
        self.variable_wirte(var);
      }

      asts::Types::Call(fun) => {
        self.function_write(&fun);
      }

      _ => {
        return;
      }
    }
  }
}
