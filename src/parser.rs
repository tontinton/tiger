use std::any::Any;

use typed_arena::Arena;

use crate::ast::Expression;
use crate::lexer::Lexer;
use crate::parser_scope::ParserScope;
use crate::token::{Token, TokenType};

pub struct Parser<'a, 'b, 'c> {
    lexer: &'a mut Lexer,
    scope: &'b mut ParserScope,
    arena: &'c Arena<Expression<'c>>,
    stop_at: char,
}

type ExprOption<'c> = Option<&'c Expression<'c>>;

impl<'a, 'b, 'c> Parser<'a, 'b, 'c> {
    pub fn new(lexer: &'a mut Lexer, scope: &'b mut ParserScope, arena: &'c Arena<Expression<'c>>) -> Self {
        Self {
            lexer,
            scope,
            arena,
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

    fn get_special_char_expression(&mut self, option_prev: ExprOption<'c>, token: Token) -> ExprOption<'c> {
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
                        }
                    };

                    if prev_token.typ.type_id() != TokenType::Symbol.type_id() {
                        println!("Error: assignment: the expression `{}` is not a valid symbol", prev_token.value);
                        None
                    } else {
                        if let Some(next) = self.next_expression(None) {
                            Some(self.arena.alloc(Expression::Operation(
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

    fn get_operation_expression(&mut self, option_prev: ExprOption<'c>, token: Token) -> ExprOption<'c> {
        match self.next_expression(None) {
            Some(subtree) => {
                let priority = Parser::get_operation_priority(&token);

                if let Some(prev) = option_prev {
                    if let Expression::Operation(left, subtree_token, right) = subtree {
                        let subtree_priority = Parser::get_operation_priority(&subtree_token);
                        if priority > 0 && subtree_priority > 0 && priority > subtree_priority {
                            // Create a rotated left operation tree
                            let new_left = self.arena.alloc(Expression::Operation(prev, token, left));
                            Some(self.arena.alloc(Expression::Operation(new_left,
                                                                        subtree_token.clone(),
                                                                        right)))
                        } else {
                            let new_right = self.arena.alloc(Expression::Operation(left,
                                                                                   subtree_token.clone(),
                                                                                   right));
                            Some(self.arena.alloc(Expression::Operation(prev, token, new_right)))
                        }
                    } else {
                        Some(self.arena.alloc(Expression::Operation(prev, token, subtree)))
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

    fn get_variable_assignment_expression(&mut self, option_prev: ExprOption<'c>) -> ExprOption<'c> {
        if option_prev.is_some() {
            println!("Error: `let [name] : [type]`, `let` must at the beginning of the expression");
            return None;
        }
        if let Some(name_token) = self.eat_token() {
            match name_token.typ {
                TokenType::Symbol => {
                    if let Some(colon_token) = self.eat_token() {
                        match colon_token.typ {
                            TokenType::Colon => {
                                if let Some(type_token) = self.eat_token() {
                                    match type_token.typ {
                                        TokenType::Symbol => {
                                            let variable = self.arena.alloc(Expression::Literal(name_token.clone()));
                                            let typ = self.arena.alloc(Expression::Literal(type_token));
                                            if let Some(value) = self.next_expression(Some(variable)) {
                                                // TODO: check that value is actually an assignment expression
                                                self.scope.variables.push(name_token.value);
                                                Some(self.arena.alloc(Expression::VariableDeclaration(variable, typ, value)))
                                            } else {
                                                println!("Error: `let [name] : [type]`, no assignment expression found for the variable");
                                                None
                                            }
                                        }
                                        _ => {
                                            println!("Error: `let [name] : [type]`, [type] given after `:` is not a valid symbol");
                                            None
                                        }
                                    }
                                } else {
                                    println!("Error: `let [name] : [type]`, expression ended prematurely, [type] not found");
                                    None
                                }
                            }
                            _ => {
                                println!("Error: `let [name] : [type]`, `:` did not come after [name]");
                                None
                            }
                        }
                    } else {
                        println!("Error: `let [name] : [type]`, expression ended prematurely, nothing came after [name]");
                        None
                    }
                }
                _ => {
                    println!("Error: `let [name] : [type]`, the [name] given is not a valid symbol");
                    None
                }
            }
        } else {
            println!("Error: `let [name] : [type]`, expression ended prematurely, [name] not found");
            None
        }
    }

    fn next_body_until_char(&mut self, stop_at: char) -> ExprOption<'c> {
        let mut parser = Parser::new(self.lexer, self.scope, self.arena);
        parser.stop_at = stop_at;
        parser.parse()
    }

    fn next_expression_until_char(&mut self, stop_at: char) -> ExprOption<'c> {
        let prev_stop_at = self.stop_at;

        self.stop_at = stop_at;
        let expression = self.next_expression(None);
        self.stop_at = prev_stop_at;

        expression
    }

    fn next_expression(&mut self, option_prev: ExprOption<'c>) -> ExprOption<'c> {
        let token = match self.eat_token() {
            Some(x) => x,
            None => return None,
        };

        match token.typ {
            TokenType::Special => self.get_special_char_expression(option_prev, token),
            TokenType::Operation => self.get_operation_expression(option_prev, token),
            TokenType::Symbol => {
                if self.scope.variables.contains(&token.value) {
                    self.next_expression(Some(self.arena.alloc(Expression::Literal(token))))
                } else {
                    println!("Error: `{}` is not declared", token.value);
                    None
                }
            }
            TokenType::If => {
                if let Some(condition) = self.next_expression_until_char('{') {
                    if let Some(then) = self.next_body_until_char('}') {
                        let if_expr = self.arena.alloc(Expression::IfThen(condition, then));
                        return if let Some(next_expr) = self.next_expression(Some(if_expr)) {
                            Some(next_expr)
                        } else {
                            Some(if_expr)
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
                                Some(self.arena.alloc(Expression::IfElseThen(condition,
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
                self.next_expression(Some(self.arena.alloc(Expression::Literal(token))))
            }
            TokenType::Let => {
                self.get_variable_assignment_expression(option_prev)
            }
            _ => {
                println!("Error: failed to parse token: {}", token.value);
                None
            }
        }
    }

    pub fn parse(mut self) -> ExprOption<'c> {
        let mut expression_list: Vec<&Expression> = vec![];
        while let Some(expression) = self.next_expression(None) {
            expression_list.push(expression);
        }
        if expression_list.len() > 0 {
            Some(self.arena.alloc(Expression::Body(expression_list)))
        } else {
            None
        }
    }
}
