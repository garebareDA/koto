use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;

fn main() {
    let index = 0;
    let path = "./programs/fuga.koto";
    let file = File::open(path).expect("file not found");
    let mut file_buffer = BufReader::new(&file);
    let mut content = String::new();
    file_buffer.read_to_string(&mut content);

    get(&content, index);
}

fn get(content: &str, mut index: usize) -> usize {
    let one_char = content.chars().nth(index);
    while one_char == Some(' ') || one_char == Some('\n') {
        index += 1;
    }

    let len = content.len();
    if index as usize >= len as usize {
        return 0;
    }

    let last_str = content.chars().nth(index).expect("Failed").to_string();
    let mut identifier_str: String = String::new();

    let reg = Regex::new(r"[a-zA-Z]+").expect("Failed to create REGEX");;
    match reg.captures(&last_str){
        _ => {}
    }

    return 0;
}