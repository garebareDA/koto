pub mod ast;
pub mod compiler;
pub mod interpreter;
pub mod lexer;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/import.js")]
extern "C" {
    pub fn output_result(input: &str);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run(string:&str) {
  output_result("Run......\n");
  let mut lexer = lexer::lexers::Lexer::new(string);
  let tokens = lexer.start();

  let mut pars = ast::parsing::Parsing::new(&tokens);
  let result = pars.parsing();

  interpreter::interpreters::run(result);
  output_result("\n......Done");
}