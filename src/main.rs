use std::env;

use koto::ast::Parsing;
use koto::interpreter;
use koto::lexer::Lexer;
use koto::lexer::Token;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    if &args[1] == "run" {
        let mut index = 0;
        let path = &args[2];
        let file = File::open(path).expect("file not found");
        let mut file_buffer = BufReader::new(&file);
        let mut content = String::new();
        file_buffer.read_to_string(&mut content);
        let len = content.len();
        let mut tokens: Vec<Token::TokenValue> = Vec::new();
        loop {
            if index >= len {
                break;
            }
            let (result, continue_index) = Lexer::get(&content, index);
            index = continue_index;
            println!("{:?}", result);
            tokens.push(result);
        }
        let result = Parsing::parsing(&mut tokens);
        println!("{:?}", result);

        interpreter::Interpreter::run(result);
    } else {
        println!("file run");
        println!("./koto run [file name]");
    }
}
