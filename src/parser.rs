use std::any::Any;

use crate::token::{Token, TokenType};
use crate::ast::{Expression};

pub struct Parser {
    tokens: Vec<Token>,
    length: usize,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let length = tokens.len();
        Self {
            tokens,
            length,
            index: 0,
        }
    }

    fn get_operation_priority(token: &Token) -> usize {
        let c = token.value.as_bytes()[0] as char;
        match c {
            '+' | '=' => 1,
            '*' | '/' => 2,
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

    fn next_expression(&mut self, option_prev: Option<Box<Expression>>) -> Option<Box<Expression>> {
        let token = match self.eat_token() {
            Some(x) => x,
            None => return None,
        };

        let token_value = token.value.clone();

        let type_id = token.typ.type_id();
        if option_prev.is_none() && (type_id == TokenType::Number.type_id() || type_id == TokenType::Symbol.type_id()) {
            let simple_node = Expression::Literal(token);
            return self.next_expression(Some(Box::new(simple_node)));
        };

        let result_tree = match option_prev {
            Some(prev) => {
                match token.typ {
                    TokenType::Special => {
                        let c = token.value.as_bytes()[0] as char;
                        match c {
                            ';' => Some(prev),
                            '=' => {
                                if let Some(prev_token) = prev.get_token() {
                                    if prev_token.typ.type_id() != TokenType::Symbol.type_id() {
                                        println!("Error: assignment: {} is not a valid symbol", prev_token.value);
                                        None
                                    } else {
                                        if let Some(next) = self.next_expression(None) {
                                            Some(Box::new(Expression::Operation(
                                                prev,
                                                Token { typ: TokenType::Assignment, value: token.value.clone() },
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
                            }
                            _ => {
                                println!("Error: special: found an unknown character");
                                None
                            }
                        }
                    }

                    TokenType::Operation => {
                        match self.next_expression(None) {
                            Some(subtree) => {
                                let priority = Parser::get_operation_priority(&token);

                                if let Expression::Operation(left, subtree_token, right) = *subtree {
                                    let subtree_priority = Parser::get_operation_priority(&subtree_token);
                                    if priority > 0 && subtree_priority > 0 && priority > subtree_priority {
                                        // let operation_tree = Tree { token, left: prev, right: x };

                                        // Create a rotated left operation tree
                                        Some(Box::new(Expression::Operation(
                                            Box::new(Expression::Operation(prev, token, left)),
                                            subtree_token,
                                            right)))
                                    } else {
                                        Some(Box::new(Expression::Operation(
                                            prev,
                                            token,
                                            Box::new(Expression::Operation(left, subtree_token, right))))
                                        )
                                    }
                                } else {
                                    Some(Box::new(Expression::Operation(prev, token, subtree)))
                                }
                            }
                            None => {
                                println!("Error: operation: could not find an expression after {}", token_value);
                                None
                            }
                        }
                    }
                    _ => {
                        println!("Error: found an unknown token");
                        None
                    }
                }
            }
            _ => None
        };

        if result_tree.is_none() {
            println!("Error: on token: {}", token_value);
        }
        result_tree
    }
}

impl Iterator for Parser {
    type Item = Box<Expression>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_expression(None)
    }
}
