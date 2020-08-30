use std::{fs, io};

use crate::token::{Token, TokenType};
use std::path::Path;

pub struct Lexer {
    text: String,
    length: usize,
    index: usize,
    prev_new_line_index: usize,
    next_new_line_index: Option<usize>,
    current_line_index: usize,
}

impl Lexer {
    pub fn from_str(text: &str) -> Self {
        let length = text.len();
        let mut lexer = Self {
            text: text.to_string(),
            length,
            index: 0,
            prev_new_line_index: 0,
            next_new_line_index: Some(0),
            current_line_index: 0,
        };
        lexer.update_current_line();
        lexer
    }

    pub fn from_file(path: &Path) -> io::Result<Self> {
        let text = fs::read_to_string(path)?;
        Ok(Self::from_str(&*text))
    }

    pub fn get_current_line(&self) -> (usize, &str) {
        if let Some(index) = self.next_new_line_index {
            (self.current_line_index, &self.text[self.prev_new_line_index..index])
        } else {
            (self.current_line_index, &self.text[self.prev_new_line_index..])
        }
    }

    fn update_current_line(&mut self) {
        if let Some(index) = self.next_new_line_index {
            self.prev_new_line_index = index;
        }
        if let Some(next_new_line_offset) = self.text[self.index..].find('\n') {
            self.next_new_line_index = Some(self.index + next_new_line_offset + 1);
        } else {
            self.next_new_line_index = None;
        }
        self.current_line_index += 1;
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
            if self.index >= self.length {
                break;
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
                'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => self.index += 1,
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

        let c = self.eat_char()?;
        match c {
            ' ' | '\r' | '\t' => self.next(),
            '\n' => {
                self.update_current_line();
                self.next()
            }
            '-' => {
                if let Some(next_c) = self.eat_char() {
                    if next_c == '>' {
                        return Some(Token { typ: TokenType::SmallArrow, value: "->".to_string() });
                    } else {
                        self.index -= 1;
                    }
                }
                Some(Token { typ: TokenType::Operation, value: c.to_string() })
            }
            '+' | '/' | '*' => Some(Token { typ: TokenType::Operation, value: c.to_string() }),
            '>' | '<' => {
                if let Some(next_c) = self.eat_char() {
                    if next_c == '=' {
                        return Some(Token { typ: TokenType::Operation, value: format!("{}{}", c, next_c) });
                    } else {
                        self.index -= 1;
                    }
                }
                Some(Token { typ: TokenType::Operation, value: c.to_string() })
            }
            '=' => {
                if let Some(next_c) = self.eat_char() {
                    if next_c == '=' {
                        return Some(Token { typ: TokenType::Operation, value: "==".to_string() });
                    } else {
                        self.index -= 1;
                    }
                }
                Some(Token { typ: TokenType::Special, value: c.to_string() })
            }
            ':' => {
                if let Some(next_c) = self.eat_char() {
                    if next_c == '=' {
                        return Some(Token { typ: TokenType::Walrus, value: ":=".to_string() });
                    } else {
                        self.index -= 1;
                    }
                }
                Some(Token { typ: TokenType::Colon, value: c.to_string() })
            }
            ';' | '{' | '}' | '(' | ')' | ',' => Some(Token { typ: TokenType::Special, value: c.to_string() }),
            '0'..='9' | '.' => {
                self.index -= 1;
                let x = self.eat_number()?;
                Some(Token { typ: TokenType::Number, value: x })
            }
            'A'..='Z' | 'a'..='z' => {
                self.index -= 1;
                let x = self.eat_string()?;
                match x.as_str() {
                    "if" => Some(Token { typ: TokenType::If, value: x }),
                    "else" => Some(Token { typ: TokenType::Else, value: x }),
                    "let" => Some(Token { typ: TokenType::Let, value: x }),
                    "fn" => Some(Token { typ: TokenType::Func, value: x }),
                    "return" => Some(Token { typ: TokenType::Return, value: x }),
                    _ => Some(Token { typ: TokenType::Symbol, value: x }),
                }
            }
            _ => None,
        }
    }
}
