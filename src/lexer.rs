use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    content: &'a [u8],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [u8]) -> Self {
        Lexer { content }
    }

    pub fn lex(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut current_index: usize = 0;
        while current_index < self.content.len() {
            let mut buffer: String = String::from("");
            // single char tokens
            if let Some(token_type) = match self.content[current_index] {
                b'{' => Some(TokenType::LeftCurlyBracket),
                b'}' => Some(TokenType::RightCurlyBracket),
                b'[' => Some(TokenType::LeftBracket),
                b']' => Some(TokenType::RightBracket),
                b':' => Some(TokenType::Colon),
                b',' => Some(TokenType::Comma),
                _ => None,
            } {
                tokens.push(Token::new(token_type, None));
                current_index+=1;
                continue;
            }

            // strings(all keys should have ""?)

            // numbers

            // booleans
            
            // null

        }
        tokens
    }
}
