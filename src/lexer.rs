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

            // we should ignore newlines, tabs and white spaces
            if match self.content[current_index] {
                b'\t' => true,
                b'\n' => true,
                b' ' => true,
                _ => false,
            } {
                continue;
            }

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
                current_index += 1;
                continue;
            }

            // strings(all keys should have ""?)
            if self.content[current_index] == b'"' {
                current_index += 1;
                while current_index < self.content.len() && self.content[current_index] != b'"' {
                    buffer.push(self.content[current_index] as char);
                    current_index += 1;
                }
                // we didn't find another '"'
                if current_index >= self.content.len() {
                    panic!(r#"expect '"' to always have a match"#);
                }
                tokens.push(Token::new(TokenType::Str, Some(buffer.clone())));
                buffer.clear();
                continue;
            }

            // numbers
            if self.content[current_index].is_ascii_digit() {
                buffer.push(self.content[current_index] as char);
                current_index += 1;
                while current_index < self.content.len()
                    && self.content[current_index].is_ascii_digit()
                {
                    buffer.push(self.content[current_index] as char);
                    current_index += 1;
                }
                // nothing more to check, we have a int and it's over
                if current_index > self.content.len() {
                    tokens.push(Token::new(TokenType::Int, Some(buffer.clone())));
                    return tokens;
                }
                // if it's not over, we could have a float,so we check for '.'
                if self.content[current_index] == b'.' {
                    buffer.push(self.content[current_index] as char);
                } else {
                    continue;
                }
            }

            // booleans

            // null
        }
        tokens
    }
}
