use std::process::exit;
use super::super::lexer::token::TokenValue;

pub struct Error {
    pub tokens: TokenValue,
}

impl Error {
    pub fn new(token: &TokenValue) -> Error {
        Error { tokens: token.clone() }
    }

    pub fn exit(self, message: &str) {
        eprintln!("{} {}",  self.tokens.val,message);
        exit(0);
    }
}

