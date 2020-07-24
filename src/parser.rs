use std::any::Any;

use crate::token::{Token, TokenType};
use crate::ast::{TreeNode, Tree};

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

    fn next_expression(&mut self, prev: TreeNode) -> TreeNode {
        let token = match self.eat_token() {
            Some(x) => x,
            None => return None,
        };

        let type_id = token.typ.type_id();
        if prev.is_none() && (type_id == TokenType::Number.type_id() || type_id == TokenType::Symbol.type_id()) {
            let simple_node = Tree { token, left: None, right: None };
            return self.next_expression(Some(Box::new(simple_node)));
        };

        let token_value = token.value.clone();

        let result_tree = match token.typ {
            TokenType::Special => {
                let c = token.value.as_bytes()[0] as char;
                match c {
                    ';' => prev,
                    '=' => {
                        let prev_token = match &prev {
                            Some(x) => x.token.clone(),
                            None => Token { typ: TokenType::Unknown, value: "?".to_string() },
                        };

                        if prev_token.typ.type_id() != TokenType::Symbol.type_id() {
                            println!("Error: assignment: {} is not a valid symbol", prev_token.value);
                            None
                        } else {
                            Some(Box::new(Tree {
                                token: Token { typ: TokenType::Assignment, value: token.value },
                                left: prev,
                                right: self.next_expression(None),
                            }))
                        }
                    }
                    _ => {
                        println!("Error: special: found an unknown character");
                        None
                    }
                }
            }

            TokenType::Operation => {
                let operation_subtree = self.next_expression(None);
                let operation_tree = Tree { token, left: prev, right: operation_subtree };

                match &operation_tree.right {
                    Some(x) => {
                        let priority = Parser::get_operation_priority(&operation_tree.token);
                        let subtree_priority = Parser::get_operation_priority(&x.token);
                        if priority > 0 && subtree_priority > 0 && priority > subtree_priority {
                            Some(Box::new(operation_tree.rotate_left()))
                        } else {
                            Some(Box::new(operation_tree))
                        }
                    }
                    None => Some(Box::new(operation_tree)),
                }
            }
            _ => {
                println!("Error: found an unknown token");
                None
            }
        };

        if result_tree.is_none() {
            println!("Error: on token: {}", token_value);
        }
        result_tree
    }
}

impl Iterator for Parser {
    type Item = Box<Tree>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_expression(None)
    }
}
