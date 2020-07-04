extern crate web_sys;
extern crate wasm_bindgen;

use koto::compiler;
use koto::interpreter;
use koto::lexer::lexers;
use koto::ast::parsing;

use std::env;
use std::process::Command;
use wasm_bindgen::prelude::*;

//関数の書き込みをなんとかする

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 && &args[1] == "run" {
        let path = &args[2];
        let result = interpreter::interpreters::read_file(path);
        println!("{:?}", result);
        interpreter::interpreters::run(result);
    } else if args.len() == 3 && &args[1] == "compile" {
        let path = &args[2];
        let result = interpreter::interpreters::read_file(path);
        println!("{:?}", result);
        let mut compiler = compiler::to_c::Compile::new();
        compiler.compile(result);

        let mut process = Command::new("gcc")
            .arg("-o")
            .arg("./build/build")
            .arg("./build/build.c")
            .spawn()
            .expect("failed to run");
        process.wait().expect("error");

        let mut run = Command::new("./build/build").spawn().expect("faild to run");
        run.wait().expect("error");
    } else {
        println!("file run");
        println!("./koto run [file name]");
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run() {
    let document = web_sys::window().unwrap().document().unwrap();
    document.get_element_by_id("result").unwrap().set_node_value(Some(""));

    let val = document.get_element_by_id("code").unwrap().node_value().unwrap();
    log(&val);

    let mut lexer = lexers::Lexer::new(&val);
    let tokens = lexer.start();
    let mut pars = parsing::Parsing::new(&tokens);
    let result = pars.parsing();
    interpreter::interpreters::run(result);
}