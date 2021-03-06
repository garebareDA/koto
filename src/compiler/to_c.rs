use super::super::ast::asts;
use super::super::interpreter::error;
use super::variable;
use std::fs;
use std::io::Write;


//変数の関数の方を調べるために配列を用意する

pub struct Compile {
  pub file: std::fs::File,
  pub variable: variable::Vriables,
  pub function: variable::Vriables,
  pub tmp:usize,
  pub inner_name: Option<String>,
  pub import: String,
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
      tmp:0,
      inner_name: None,
      import: "".to_string(),
    }
  }

  pub fn to_import(&mut self, import:&str) {
    self.import = import.to_string();
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

  pub(crate) fn scope(&mut self, ast: &Vec<asts::Types>) {
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

      asts::Types::Binary(bin) => {
        match &bin.node[0] {
          asts::Types::Variable(vars) => {
            self.calcuration(bin, &vars.name);
            self.write("\n");
          }
          _ => {
            let err = error::Error::new(node);
            err.exit("type error");
          }
        }
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

      asts::Types::Retrun(rets) => match self.inner_name.clone() {
        Some(n) => {
          self.return_write(rets, &n);
        },

        None => {
          self.write("return 0;");
        }
      },

      _ => {
        return;
      }
    }
  }
}
