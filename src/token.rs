#[derive(Debug, Clone)]
pub enum TokenType {
    // {
    LeftCurlyBracket,
    // }
    RightCurlyBracket,
    // [
    LeftBracket,
    // ]
    RightBracket,
    // a string
    Str,

    // a integer
    Int,

    // a float
    Float,

    // a boolean
    Bool,

    // the null value
    Null,

    // :
    Colon,

    // ,
    Comma,
}

pub struct Token {
    _type: TokenType,
    value: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>) -> Self {
        Token {
            _type: token_type,
            value: value,
        }
    }
}
