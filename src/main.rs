use std::env;

use koto::ast::parsing;
use koto::interpreter;
use koto::lexer::lexers;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 && &args[1] == "run" {
        let path = &args[2];
        let file = File::open(path).expect("file not found");
        let mut file_buffer = BufReader::new(&file);
        let mut content = String::new();
        file_buffer.read_to_string(&mut content).expect("file not found");

        let mut lexer = lexers::Lexer::new(&content);
        let tokens = lexer.start();

        let mut pars = parsing::Parsing::new(&tokens);
        let result = pars.parsing();
        println!("{:?}", result);

        interpreter::interpreters::run(result);
    } else {
        println!("file run");
        println!("./koto run [file name]");
    }
}
