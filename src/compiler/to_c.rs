use super::super::ast::asts;
use super::variable;
use std::fs;
use std::io::Write;

//変数の関数の方を調べるために配列を用意する

pub struct Compile {
  pub file: std::fs::File,
  pub variable: variable::Vriables,
  pub function: variable::Vriables,
}

impl Compile {
  pub fn new() -> Compile {
    fs::create_dir_all("./build").expect("dir create failed");
    let file = fs::File::create("./build/build.c").expect("create failed");
    let vairables = variable::Vriables::new();
    let function = variable::Vriables::new();
    Compile {
      file: file,
      variable: vairables,
      function: function,
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
    self.function_write(&root.node);
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

  pub(crate) fn scope(&mut self, ast: &Vec<asts::Types>,) {
    let mut index = 0;
    let len = ast.len();
    loop {
      if index >= len {
        break;
      }

      let node = &ast[index];
      self.judge(node);
      index += 1;
    }
  }

  fn judge(&mut self, node: &asts::Types) {
    match node {
      asts::Types::Variable(var) => {
        self.variable_wirte(var);
        self.write("\n");
      }

      asts::Types::Call(fun) => {
        self.call_write(fun);
      }

      asts::Types::If(ifs) => {
        self.ifs_write(ifs);
      }

      asts::Types::For(fors) => {
        self.fors_write(&fors);
      }

      _ => {
        return;
      }
    }
  }
}
