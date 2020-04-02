use super::token;
use regex::Regex;

static TOKEN: token::Token = token::Token::new();

pub struct Lexer {
    pub string: String,
    pub index: usize,
}

impl Lexer {
    pub fn new(string: &str) -> Lexer {
        let string = string.to_string();
        Lexer {
            string: string,
            index: 0,
        }
    }

    pub fn start(&mut self) -> Vec<token::TokenValue> {
        let content = &self.string;
        let len = self.string.len();
        let mut tokens: Vec<token::TokenValue> = Vec::new();
        loop {
            if self.index >= len {
                break;
            }
            let (result, continue_index) = Lexer::get(&content, self.index);
            self.index = continue_index;
            println!("{:?}", result);
            tokens.push(result);
        }

        return tokens;
    }

    fn get(content: &str, mut index: usize) -> (token::TokenValue, usize) {
        let mut one_char = content.chars().nth(index);

        while one_char == Some('\n') || one_char == Some(' ') {
            index += 1;
            one_char = content.chars().nth(index);
        }
        let len = content.len();
        if index >= len {
            return (token::TokenValue::new(0, ""), index);
        }
        //予約語
        let last_str = content.chars().nth(index).expect("Failed").to_string();
        let mut identifier_str: String = String::new();
        let reg = Regex::new(r"[a-zA-Z]+").expect("Failed");
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
                    let token_value = token::TokenValue::new(TOKEN._print, &identifier_str);
                    return (token_value, index);
                }
                if identifier_str == "fn" {
                    let token_value = token::TokenValue::new(TOKEN._fun, &identifier_str);
                    return (token_value, index);
                }
                if identifier_str == "if" {
                    let token_value = token::TokenValue::new(TOKEN._if, &identifier_str);
                    return (token_value, index);
                }
                if identifier_str == "else" {
                    let token_value = token::TokenValue::new(TOKEN._else, &identifier_str);
                    return (token_value, index);
                }
                if identifier_str == "for" {
                    let token_value = token::TokenValue::new(TOKEN._for, &identifier_str);
                    return (token_value, index);
                }
                if identifier_str == "let" {
                    let token_value = token::TokenValue::new(TOKEN._let, &identifier_str);
                    return (token_value, index);
                }
                if identifier_str == "true" || identifier_str == "false" {
                    let token_value = token::TokenValue::new(TOKEN._bool, &identifier_str);
                    return (token_value, index);
                }
                if identifier_str == "return" {
                    let token_value = token::TokenValue::new(TOKEN._return, &identifier_str);
                    return (token_value, index);
                }
                if identifier_str == "vec" {
                    let token_value = token::TokenValue::new(TOKEN._vec, &identifier_str);
                    return (token_value, index);
                }
                if identifier_str == "import"{
                    let token_value = token::TokenValue::new(TOKEN._import, &identifier_str);
                    return (token_value, index);
                }
                let token_value = token::TokenValue::new(TOKEN._identifier, &identifier_str);
                return (token_value, index);
            }
            None => {}
        };
        //文字列
        let reg = Regex::new(r#"""#).expect("Faild");
        match reg.captures(&last_str) {
            Some(_) => {
                identifier_str = String::new();
                loop {
                    index += 1;
                    let text = &content.chars().nth(index).expect("Failed").to_string();
                    if text != "\"" {
                        identifier_str += &text;
                    }
                    if text == "\"" {
                        break;
                    };
                }
                let token_value = token::TokenValue::new(TOKEN._string, &identifier_str);
                return (token_value, index + 1);
            }
            None => {}
        };
        //数字
        let reg = Regex::new(r"[0-9]+").expect("Faild");
        match reg.captures(&last_str) {
            Some(_) => {
                loop {
                    let text = &content.chars().nth(index).expect("Failed").to_string();
                    let reg = Regex::new(r"[0-9.]+").expect("Faild");
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
                let token_value = token::TokenValue::new(TOKEN._number, &identifier_str);
                return (token_value, index);
            }
            None => {}
        };
        //コメント
        let reg = Regex::new(r#"#"#).expect("Faild");
        match reg.captures(&last_str) {
            Some(_) => {
                loop {
                    let text = &content.chars().nth(index).expect("Failed").to_string();
                    if text == "\n" {
                        break;
                    }
                    identifier_str += text;
                    index += 1;
                }
                let token_value = token::TokenValue::new(TOKEN._comment, &identifier_str);
                return (token_value, index);
            }
            None => {}
        }

        let ascii_code = content
            .chars()
            .nth(index)
            .expect("Failed")
            .to_string()
            .as_bytes()[0];
        let token_value = token::TokenValue::new(ascii_code as i64, &last_str);
        return (token_value, index + 1);
    }
}
