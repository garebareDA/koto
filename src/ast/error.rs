use std::process::exit;
use super::super::lexer::token::TokenValue;
use wasm_bindgen::prelude::*;

pub struct Error {
    pub tokens: TokenValue,
}

impl Error {
    pub fn new(token: &TokenValue) -> Error {
        Error { tokens: token.clone() }
    }

    pub fn exit(self, message: &str) {
        println!("{} {}",  self.tokens.val,message);
        output_result(message);
        exit(0);

        #[wasm_bindgen(module = "/js/import.js")]
        extern "C" {
            pub fn output_result(input: &str);
        }
    }
}