use crate::token::{Token, TokenType};

pub struct Lexer {
    text: String,
    length: usize,
    index: usize,
}

impl Lexer {
    pub fn new(text: &str) -> Self {
        let length = text.len();
        Self {
            text: text.to_string(),
            length,
            index: 0,
        }
    }

    fn peek_char(&mut self) -> char {
        self.text.as_bytes()[self.index] as char
    }

    fn eat_char(&mut self) -> Option<char> {
        if self.index >= self.length {
            return None;
        }

        let current_char = self.peek_char();
        self.index += 1;
        Some(current_char)
    }

    fn eat_number(&mut self) -> Option<String> {
        if self.index >= self.length {
            return None;
        }

        let start_index = self.index;

        let mut current_char = self.peek_char();
        loop {
            match current_char {
                '0'..='9' | '.' => self.index += 1,
                _ => break,
            }
            current_char = self.peek_char();
        }

        Some(self.text[start_index..self.index].to_string())
    }

    fn eat_string(&mut self) -> Option<String> {
        if self.index >= self.length {
            return None;
        }

        let start_index = self.index;

        let mut current_char = self.peek_char();
        loop {
            match current_char {
                'A'..='Z' | 'a'..='z' | '0'..='9' => self.index += 1,
                _ => break,
            }
            current_char = self.peek_char();
        }

        Some(self.text[start_index..self.index].to_string())
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.length {
            return None;
        }
        let char_eaten = self.eat_char();
        match char_eaten {
            Some(c) => {
                match c {
                    ' ' | '\n' => self.next(),
                    '+' | '-' | '/' | '*' => Some(Token { typ: TokenType::Operation, value: c.to_string() }),
                    ';' | ':' | '=' => Some(Token { typ: TokenType::Special, value: c.to_string() }),
                    '0'..='9' | '.' => {
                        self.index -= 1;
                        let value = self.eat_number();
                        match value {
                            Some(x) => Some(Token { typ: TokenType::Number, value: x }),
                            None => None,
                        }
                    }
                    'A'..='Z' | 'a'..='z' => {
                        self.index -= 1;
                        let value = self.eat_string();
                        match value {
                            Some(x) => Some(Token { typ: TokenType::Symbol, value: x }),
                            None => None,
                        }
                    }
                    _ => None,
                }
            }
            None => None,
        }
    }
}
