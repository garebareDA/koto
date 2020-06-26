use super::to_c::Compile;
use super::super::ast::asts;
use super::super::interpreter::error;

impl Compile {
  pub(crate) fn fors_write(&mut self, fors: &asts::ForAST){
    self.variable.vec_push();
    self.function.vec_push();

    let variant = &fors.init_var[0];
    let judge = &fors.init_var[1];
    let loop_for = &fors.init_var[2];

    self.write("for(");

    match variant {
      asts::Types::Variable(vars) => {
        self.variable_wirte(vars);
      }

      _ => {
        let err = error::Error::new(variant);
        err.exit("for init error");
      }
    }

    match judge {
      asts::Types::Binary(bin) => {
        self.fors_juge(&bin.node, Some(bin.op));
        self.write(";");
      }

      _ => {
        let err = error::Error::new(variant);
        err.exit("for init error");
      }
    }

    match loop_for {
      asts::Types::Binary(bin) => {
        self.fors_juge(&bin.node, Some(bin.op));
      }

      _ => {
        let err = error::Error::new(loop_for);
        err.exit("loop error");
      }
    }

    self.write(")\n");
    self.write("{\n");
    self.scope(&fors.node);
    self.write("}\n");

    self.variable.last_remove();
    self.function.last_remove();
  }

  fn fors_juge(&mut self, node:&Vec<asts::Types>, op:Option<char>) {
    if node.is_empty() {
      return;
    }

    match &node[0] {
      asts::Types::Binary(bin) => {
        self.write(&bin.op.to_string());
        self.fors_juge(&bin.node, None);
      }

      asts::Types::Boolean(bools) => {
        if bools.boolean {
          self.write("1");
        }else{
          self.write("0");
        }
      }

      asts::Types::Variable(vars) => {
        self.write(&vars.name);
        self.fors_juge(&vars.node, None);
      }

      asts::Types::Number(num) => {
        self.write(&num.val.to_string());
        self.fors_juge(&num.node, None);
      }

      _ => {
        let err = error::Error::new( &node[0]);
        err.exit("judge error");
      }
    }

    match op {
      Some(o) => {
        self.write(&o.to_string());
      }

      None => {
        return;
      }
    }

    if node.len() == 2 {
      match &node[1] {
        asts::Types::Binary(bin) => {
          self.write(&bin.op.to_string());
          self.fors_juge(&bin.node, None);
        }

        asts::Types::Number(num) => {
          self.write(&num.val.to_string());
          self.fors_juge(&num.node, None);
        }

        _ => {
          let err = error::Error::new( &node[1]);
          err.exit("judge error");
        }
      }
    }
  }
}