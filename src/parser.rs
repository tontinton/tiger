use std::any::Any;

use crate::ast::Expression;
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    length: usize,
    index: usize,
    stop_at: Option<char>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let length = tokens.len();
        Self {
            tokens,
            length,
            index: 0,
            stop_at: None,
        }
    }

    fn get_operation_priority(token: &Token) -> usize {
        let c = token.value.as_bytes()[0] as char;
        match c {
            '>' | '<' | '=' => 1,
            '+' | '-' => 2,
            '*' | '/' => 3,
            _ => 0,
        }
    }

    fn eat_token(&mut self) -> Option<Token> {
        if self.index >= self.length {
            return None;
        }
        let token = self.tokens[self.index].clone();
        self.index += 1;
        Some(token)
    }

    fn symbol_token_eaten(&mut self, value: &str) -> bool {
        if self.index >= self.length {
            return false;
        }
        let token = self.tokens[self.index].clone();
        if token.typ.type_id() == TokenType::Symbol.type_id() && token.value == value {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn get_special_char_expression(&mut self, option_prev: Option<Box<Expression>>, token: Token) -> Option<Box<Expression>> {
        let c = token.value.as_bytes()[0] as char;
        if let Some(stop_at) = self.stop_at {
            if c == stop_at {
                return option_prev;
            }
        }

        match c {
            ';' => option_prev,
            '=' => {
                if let Some(prev) = option_prev {
                    if let Some(prev_token) = prev.get_token() {
                        if prev_token.typ.type_id() != TokenType::Symbol.type_id() {
                            println!("Error: assignment: {} is not a valid symbol", prev_token.value);
                            None
                        } else {
                            if let Some(next) = self.next_expression(None) {
                                Some(Box::new(Expression::Operation(
                                    prev,
                                    Token { typ: TokenType::Assignment, value: token.value },
                                    next,
                                )))
                            } else {
                                None
                            }
                        }
                    } else {
                        println!("Error: assignment: could not get a token from prev");
                        None
                    }
                } else {
                    println!("Error: assignment: no expression before `=`");
                    None
                }
            }
            '{' => self.next_body_until_char('}'),
            _ => {
                println!("Error: special: found an unknown character");
                None
            }
        }
    }

    fn get_operation_expression(&mut self, option_prev: Option<Box<Expression>>, token: Token) -> Option<Box<Expression>> {
        match self.next_expression(None) {
            Some(subtree) => {
                let priority = Parser::get_operation_priority(&token);

                if let Some(prev) = option_prev {
                    if let Expression::Operation(left, subtree_token, right) = *subtree {
                        let subtree_priority = Parser::get_operation_priority(&subtree_token);
                        if priority > 0 && subtree_priority > 0 && priority > subtree_priority {
                            // Create a rotated left operation tree
                            Some(Box::new(Expression::Operation(Box::new(Expression::Operation(prev, token, left)),
                                                                subtree_token,
                                                                right)))
                        } else {
                            Some(Box::new(Expression::Operation(prev,
                                                                token,
                                                                Box::new(Expression::Operation(left, subtree_token, right)))))
                        }
                    } else {
                        Some(Box::new(Expression::Operation(prev, token, subtree)))
                    }
                } else {
                    println!("Error: operation: no expression found before: {}", token.value);
                    None
                }
            }
            None => {
                println!("Error: operation: no expression found after {}", token.value);
                None
            }
        }
    }

    fn next_body_until_char(&mut self, stop_at: char) -> Option<Box<Expression>> {
        // TODO: Fix very bad performance copy
        let mut parser = Parser::new(Vec::from(&self.tokens[self.index..]));
        parser.stop_at = Some(stop_at);
        let mut expression_list: Vec<Box<Expression>> = vec![];
        while let Some(expression) = parser.next_expression(None) {
            expression_list.push(expression);
        }
        self.index += parser.index;
        if expression_list.len() > 0 {
            Some(Box::new(Expression::Body(expression_list)))
        } else {
            None
        }
    }

    fn next_expression_until_char(&mut self, stop_at: char) -> Option<Box<Expression>> {
        let prev_stop_at = self.stop_at;

        self.stop_at = Some(stop_at);
        let expression = self.next_expression(None);
        self.stop_at = prev_stop_at;

        expression
    }

    fn next_expression(&mut self, option_prev: Option<Box<Expression>>) -> Option<Box<Expression>> {
        let token = match self.eat_token() {
            Some(x) => x,
            None => return None,
        };

        match token.typ {
            TokenType::Special => self.get_special_char_expression(option_prev, token),
            TokenType::Operation => self.get_operation_expression(option_prev, token),
            TokenType::Symbol => {
                if token.value == "if" {
                    if let Some(condition) = self.next_expression_until_char('{') {
                        if let Some(then) = self.next_body_until_char('}') {
                            if self.symbol_token_eaten("else") {
                                if let Some(else_expr) = self.next_expression(None) {
                                    return Some(Box::new(Expression::IfElseThen(condition, then, else_expr)));
                                }
                            }
                            return Some(Box::new(Expression::IfThen(condition, then)));
                        } else {
                            println!("Error: if: empty body");
                        }
                    } else {
                        println!("Error: if: empty condition");
                    }
                    None
                } else {
                    let simple_node = Expression::Literal(token);
                    self.next_expression(Some(Box::new(simple_node)))
                }
            }
            TokenType::Number => {
                let simple_node = Expression::Literal(token);
                self.next_expression(Some(Box::new(simple_node)))
            }
            _ => {
                println!("Error: failed to parse token: {}", token.value);
                None
            }
        }
    }
}

impl Iterator for Parser {
    type Item = Box<Expression>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_expression(None)
    }
}
