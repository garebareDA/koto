pub struct Token {
    pub _if: i64,
    pub _then: i64,
    pub _else: i64,

    pub _for: i64,
    pub _fun: i64,
    pub _print: i64,

    pub _string: i64,
    pub _number: i64,
    pub _comment: i64,
    pub _identifier: i64,
    pub _let: i64,

    pub _bool:i64,
}

impl Token {
    pub const fn new() -> Token {
        Token {
            _if: -1,
            _then: -2,
            _else: -3,
            _for: -4,
            _fun: -5,
            _print: -6,
            _string: -7,
            _number: -8,
            _comment: -9,
            _identifier: -10,
            _let: -11,
            _bool:-12,
        }
    }
}

#[derive(Debug)]
pub struct TokenValue {
    pub token: i64,
    pub val: String,
}

impl TokenValue {
    pub fn new(token:i64, val:&str) -> TokenValue {
        let value = val.to_string();
        TokenValue{
            token:token,
            val:value,
        }
    }
}