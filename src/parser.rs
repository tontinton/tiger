use std::any::Any;

use crate::ast::Expression;
use crate::token::{Token, TokenType};
use crate::lexer::Lexer;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    stop_at: char,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        Self {
            lexer,
            stop_at: ';',
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
        self.lexer.next()
    }

    fn get_special_char_expression(&mut self, option_prev: Option<Box<Expression>>, token: Token) -> Option<Box<Expression>> {
        let c = token.value.as_bytes()[0] as char;
        if c == self.stop_at {
            return option_prev;
        }

        match c {
            ';' => option_prev, // a semicolon should always stop from reading more expressions
            '=' => {
                if let Some(prev) = option_prev {
                    let prev_token = match &(*prev) {
                        Expression::Literal(token) => token,
                        Expression::Operation(_left, token, _right) => token,
                        _ => {
                            println!("Error: assignment: the previous expression must either be a literal or an operation");
                            return None;
                        },
                    };

                    if prev_token.typ.type_id() != TokenType::Symbol.type_id() {
                        println!("Error: assignment: the expression `{}` is not a valid symbol", prev_token.value);
                        None
                    } else {
                        if let Some(next) = self.next_expression(None) {
                            Some(Box::new(Expression::Operation(
                                prev,
                                Token { typ: TokenType::Assignment, value: token.value },
                                next,
                            )))
                        } else {
                            println!("Error: assignment: no expression after `=`");
                            None
                        }
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
        let mut parser = Parser::new(&mut self.lexer);
        parser.stop_at = stop_at;
        parser.parse()
    }

    fn next_expression_until_char(&mut self, stop_at: char) -> Option<Box<Expression>> {
        let prev_stop_at = self.stop_at;

        self.stop_at = stop_at;
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
                let simple_node = Expression::Literal(token);
                self.next_expression(Some(Box::new(simple_node)))
            }
            TokenType::If => {
                if let Some(condition) = self.next_expression_until_char('{') {
                    if let Some(then) = self.next_body_until_char('}') {
                        let if_expr = Some(Box::new(Expression::IfThen(condition, then)));
                        return if let Some(next_expr) = self.next_expression(if_expr.clone()) {
                            Some(next_expr)
                        } else {
                            if_expr
                        };
                    } else {
                        println!("Error: if: empty body");
                    }
                } else {
                    println!("Error: if: empty condition");
                }
                None
            }
            TokenType::Else => {
                if let Some(prev) = option_prev {
                    match *prev {
                        Expression::IfThen(condition, then) => {
                            if let Some(else_expr) = self.next_expression(None) {
                                Some(Box::new(Expression::IfElseThen(condition,
                                                                     then,
                                                                     else_expr)))
                            } else {
                                println!("Error: else: `else` block is empty");
                                None
                            }
                        }
                        _ => {
                            println!("Error: else: must be after an `if` block");
                            None
                        }
                    }
                } else {
                    println!("Error: else: cannot be the first keyword in an expression");
                    None
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

    pub fn parse(mut self) -> Option<Box<Expression>>{
        let mut expression_list: Vec<Box<Expression>> = vec![];
        while let Some(expression) = self.next_expression(None) {
            expression_list.push(expression);
        }
        if expression_list.len() > 0 {
            Some(Box::new(Expression::Body(expression_list)))
        } else {
            None
        }
    }
}
