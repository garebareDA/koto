use super::to_c::Compile;
use super::super::ast::asts;

impl Compile {
  pub(crate) fn fors_write(&mut self, fors: &asts::ForAST){
    self.variable.vec_push();

    let variant = &fors.init_var[0];
    let judge = &fors.init_var[1];
    let loop_for = &fors.init_var[2];

    self.write("for(");

    match variant {
      asts::Types::Variable(vars) => {
        self.variable_wirte(vars);
      }

      _ => {
        //error
      }
    }

    match judge {
      asts::Types::Binary(bin) => {
        self.fors_juge(&bin.node, Some(bin.op));
        self.write(";");
      }

      _ => {
        //error
      }
    }

    match loop_for {
      asts::Types::Binary(bin) => {
        self.fors_juge(&bin.node, Some(bin.op));
      }

      _ => {
        //error
      }
    }

    self.write(")\n");
    self.write("{\n");
    self.scope(&fors.node);
    self.write("}\n");
    self.variable.last_remove();
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
        //error
      }
    }

    match op {
      Some(o) => {
        self.write(&o.to_string());
      }

      None => {}
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
          //error
        }
      }
    }
  }
}