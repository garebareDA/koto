use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;

fn main() {
    let mut index = 0;
    let path = "./programs/fuga.koto";
    let file = File::open(path).expect("file not found");
    let mut file_buffer = BufReader::new(&file);
    let mut content = String::new();
    file_buffer.read_to_string(&mut content);

    let len = content.len();
    loop{
        if index >= len {
            break;
        }

        let (result, continue_index) = get(&content, index);
        index = continue_index + 1;

        println!("{}", result);
    }
}

fn get(content: &str, mut index: usize) -> (i64, usize) {
    let len = content.len();
    if index >= len {
        return (0, index);
    }

    let last_str = content.chars().nth(index).expect("Failed").to_string();
    let mut identifier_str: String = String::new();

    let reg = Regex::new(r"[a-zA-Z]+").expect("Failed");
    match reg.captures(&last_str){
        Some(_) => {
            loop {
                let text = &content.chars().nth(index).expect("Failed").to_string();
                let reg = Regex::new(r"(\d|[a-zA-Z])+").expect("Failed");
                let res = match reg.captures(text) {
                    Some(_) => true,
                    None => false,
                };

                if !res {
                    break;
                }

                identifier_str += text;
                index += 1;
            }

            if identifier_str == "print"{return (- 1, index)}
            if identifier_str == "fn"{return (- 2, index)}
            if identifier_str == "if"{return (- 3, index)}
            if identifier_str == "else"{return (- 4, index)}
        }
        None => {}
    };
    return (0, index);
}