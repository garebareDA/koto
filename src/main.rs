use koto::lexer::Lexer;
use koto::lexer::Token;
use koto::ast::Parsing;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let mut index = 0;
    let path = "./programs/fuga.koto";
    let file = File::open(path).expect("file not found");
    let mut file_buffer = BufReader::new(&file);
    let mut content = String::new();
    file_buffer.read_to_string(&mut content);

    let len = content.len();
    let mut tokens:Vec<Token::TokenValue> = Vec::new();
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
}