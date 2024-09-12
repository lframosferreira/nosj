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
    Str(String),

    // a integer
    Int(i64),

    // a float
    Float(f64),

    // a boolean
    Bool(bool),

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
