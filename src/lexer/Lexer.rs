use regex::Regex;
use super::Token;

static TOKEN:Token::Token = Token::Token::new();

pub fn get(content: &str, mut index: usize) -> (i64, usize) {
    let mut one_char = content.chars().nth(index);

    while one_char == Some('\n') ||  one_char == Some(' '){
        index += 1;
        one_char = content.chars().nth(index);
    }

    let len = content.len();
    if index >= len {
        return (0, index);
    }

    //予約語
    let last_str = content.chars().nth(index).expect("Failed").to_string();
    let mut identifier_str: String = String::new();
    let reg = Regex::new(r"[a-zA-Z]+").expect("Failed");

    println!("char:{} index:{}", last_str, index);

    match reg.captures(&last_str) {
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

            if identifier_str == "print" {
                return (TOKEN._print, index);
            }

            if identifier_str == "fn" {
                return (TOKEN._fun, index);
            }

            if identifier_str == "if" {
                return (TOKEN._if, index);
            }

            if identifier_str == "else" {
                return (TOKEN._else, index);
            }

            if identifier_str == "for" {
                return (TOKEN._for, index);
            }

            return (TOKEN._identifier, index);
        }
        None => {}
    };

    //文字列
    let reg = Regex::new(r#"""#).expect("Faild");
    match reg.captures(&last_str) {
        Some(_) => {
            identifier_str = '"'.to_string();
            loop {
                index += 1;
                let text = &content.chars().nth(index).expect("Failed").to_string();
                identifier_str += &text;
                if text == &'"'.to_string() {
                    break;
                };
            }
            return (TOKEN._string, index + 1);
        }

        None => {}
    };

    //数字
    let reg = Regex::new(r"[0-9.]+").expect("Faild");
    match reg.captures(&last_str) {
        Some(_) => {
            loop {
                let text = &content.chars().nth(index).expect("Failed").to_string();
                let reg = Regex::new(r"[0-9.]+").expect("Faild");
                let res = match reg.captures(&text.to_string()) {
                    Some(_) => true,
                    None => false,
                };

                if !res {
                    break;
                }

                identifier_str += &text.to_string();
                index += 1;
            }

            return (TOKEN._number, index);
        }

        None => {}
    };

    let reg = Regex::new(r"#").expect("Faild");
    match reg.captures(&last_str) {
        Some(_) => {
            loop{
                let text = &content.chars().nth(index).expect("Failed").to_string();
                if text == "\n"{
                    break;
                }

                identifier_str += text;
                index += 1;
            }

            return(TOKEN._comment, index);
        }

        None => {}
    }

    let ascii_code = content.chars().nth(index).expect("Failed").to_string().as_bytes()[0];
    return (ascii_code as i64, index + 1);
}