use koto::compiler;
use koto::interpreter;
use std::env;

use std::process::{Command, Stdio};
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
