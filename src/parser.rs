use std::any::Any;

use typed_arena::Arena;

use crate::ast::Expression;
use crate::lexer::Lexer;
use crate::parser_scope::ParserScope;
use crate::token::{Token, TokenType};

type Expr<'c> = &'c Expression<'c>;
type ExprResult<'c> = Result<&'c Expression<'c>, String>;

pub struct Parser<'a, 'b, 'c> {
    lexer: &'a mut Lexer,
    scope: &'b mut ParserScope,
    arena: &'c Arena<Expression<'c>>,
    stop_at: char,
    empty_expression: Expr<'c>,
}

impl<'a, 'b, 'c> Parser<'a, 'b, 'c> {
    pub fn new(lexer: &'a mut Lexer, scope: &'b mut ParserScope, arena: &'c Arena<Expression<'c>>) -> Self {
        Self {
            lexer,
            scope,
            arena,
            stop_at: ';',
            empty_expression: arena.alloc(Expression::Empty), // TODO: delete from arena when not returning an empty expression
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

    fn is_empty_expression(&self, expression: Expr<'c>) -> bool {
        expression as *const _ == self.empty_expression as *const _
    }

    fn get_special_char_expression(&mut self, prev: Expr<'c>, token: Token) -> ExprResult<'c> {
        let c = token.value.as_bytes()[0] as char;
        if c == self.stop_at {
            return Ok(prev);
        }

        match c {
            ';' => Ok(prev), // a semicolon should always stop from reading more expressions
            '=' => {
                if self.is_empty_expression(prev) {
                    return Err("assignment: no expression before `=`".to_string());
                }
                let prev_token = match &(*prev) {
                    Expression::Literal(token) => token,
                    Expression::Operation(_left, token, _right) => token,
                    _ => {
                        return Err("assignment: the previous expression must either be a literal or an operation".to_string());
                    }
                };

                if prev_token.typ.type_id() != TokenType::Symbol.type_id() {
                    return Err(format!("assignment: the expression `{}` is not a valid symbol", prev_token.value));
                }

                let next = self.next_expression(self.empty_expression)?;
                if self.is_empty_expression(next) {
                    Err("assignment: no expression after `=`".to_string())
                } else {
                    Ok(self.arena.alloc(Expression::Operation(
                        prev,
                        Token { typ: TokenType::Assignment, value: token.value },
                        next,
                    )))
                }
            }
            '{' => self.next_body_until_char('}'),
            _ => {
                Err("special: found an unknown character".to_string())
            }
        }
    }

    fn get_operation_expression(&mut self, prev: Expr<'c>, token: Token) -> ExprResult<'c> {
        let subtree = self.next_expression(self.empty_expression)?;
        if self.is_empty_expression(subtree) {
            return Err(format!("operation: no expression found after {}", token.value));
        }

        let priority = Parser::get_operation_priority(&token);

        if self.is_empty_expression(prev) {
            Err(format!("operation: no expression found before: {}", token.value))
        } else {
            if let Expression::Operation(left, subtree_token, right) = subtree {
                let subtree_priority = Parser::get_operation_priority(&subtree_token);
                if priority > 0 && subtree_priority > 0 && priority > subtree_priority {
                    // Create a rotated left operation tree
                    let new_left = self.arena.alloc(Expression::Operation(prev, token, left));
                    Ok(self.arena.alloc(Expression::Operation(new_left,
                                                              subtree_token.clone(),
                                                              right)))
                } else {
                    let new_right = self.arena.alloc(Expression::Operation(left,
                                                                           subtree_token.clone(),
                                                                           right));
                    Ok(self.arena.alloc(Expression::Operation(prev, token, new_right)))
                }
            } else {
                Ok(self.arena.alloc(Expression::Operation(prev, token, subtree)))
            }
        }
    }

    fn build_variable_declaration_expression(&mut self, name: Token, typ: Token) -> ExprResult<'c> {
        let variable = self.arena.alloc(Expression::Literal(name.clone()));
        let typ = self.arena.alloc(Expression::Literal(typ));
        let value = self.next_expression(variable)?;
        if self.is_empty_expression(value) {
            Err("`let [name] : [type]`, no assignment expression found for the variable".to_string())
        } else {
            // TODO: check that value is actually an assignment expression
            self.scope.variables.push(name.value);
            Ok(self.arena.alloc(Expression::VariableDeclaration(variable, typ, value)))
        }
    }
    fn get_variable_declaration_expression(&mut self, prev: Expr<'c>) -> ExprResult<'c> {
        if !self.is_empty_expression(prev) {
            return Err("`let [name] : [type]`, `let` must at the beginning of the expression".to_string());
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
                                            self.build_variable_declaration_expression(name_token, type_token)
                                        }
                                        _ => {
                                            Err("`let [name] : [type]`, [type] given after `:` is not a valid symbol".to_string())
                                        }
                                    }
                                } else {
                                    Err("`let [name] : [type]`, expression ended prematurely, [type] not found".to_string())
                                }
                            }
                            TokenType::Walrus => {
                                self.build_variable_declaration_expression(name_token,
                                                                           Token { typ: TokenType::Symbol, value: "auto".to_string() })
                            }
                            _ => {
                                Err("`let [name] : [type]`, `:` or `:=` did not come after [name]".to_string())
                            }
                        }
                    } else {
                        Err("`let [name] : [type]`, expression ended prematurely, nothing came after [name]".to_string())
                    }
                }
                _ => {
                    Err("`let [name] : [type]`, the [name] given is not a valid symbol".to_string())
                }
            }
        } else {
            Err("`let [name] : [type]`, expression ended prematurely, [name] not found".to_string())
        }
    }

    fn next_body_until_char(&mut self, stop_at: char) -> ExprResult<'c> {
        let mut parser = Parser::new(self.lexer, self.scope, self.arena);
        parser.stop_at = stop_at;
        parser.parse()
    }

    fn next_expression_until_char(&mut self, stop_at: char) -> ExprResult<'c> {
        let prev_stop_at = self.stop_at;

        self.stop_at = stop_at;
        let expression = self.next_expression(self.empty_expression);
        self.stop_at = prev_stop_at;

        expression
    }

    fn next_expression(&mut self, prev: Expr<'c>) -> ExprResult<'c> {
        let token = match self.eat_token() {
            Some(x) => x,
            None => return Ok(self.empty_expression),
        };

        match token.typ {
            TokenType::Special => self.get_special_char_expression(prev, token),
            TokenType::Operation => self.get_operation_expression(prev, token),
            TokenType::Symbol => {
                if self.scope.variables.contains(&token.value) {
                    self.next_expression(self.arena.alloc(Expression::Literal(token)))
                } else {
                    Err(format!("`{}` is not declared", token.value))
                }
            }
            TokenType::If => {
                let condition = self.next_expression_until_char('{')?;
                if self.is_empty_expression(condition) {
                    return Err("if: empty condition".to_string());
                }
                let then = self.next_body_until_char('}')?;
                if self.is_empty_expression(condition) {
                    return Err("if: empty body".to_string());
                }
                let if_expr = self.arena.alloc(Expression::IfThen(condition, then));
                let next_expr = self.next_expression(if_expr)?;
                if self.is_empty_expression(next_expr) {
                    Ok(if_expr)
                } else {
                    Ok(next_expr)
                }
            }
            TokenType::Else => {
                if self.is_empty_expression(prev) {
                    return Err("else: cannot be the first keyword in an expression".to_string());
                }

                match prev {
                    Expression::IfThen(condition, then) => {
                        let else_expr = self.next_expression(self.empty_expression)?;
                        if self.is_empty_expression(else_expr) {
                            Err("else: `else` block is empty".to_string())
                        } else {
                            Ok(self.arena.alloc(Expression::IfElseThen(condition,
                                                                       then,
                                                                       else_expr)))
                        }
                    }
                    _ => {
                        Err("else: must be after an `if` block".to_string())
                    }
                }
            }
            TokenType::Number => {
                self.next_expression(self.arena.alloc(Expression::Literal(token)))
            }
            TokenType::Let => {
                self.get_variable_declaration_expression(prev)
            }
            _ => {
                Err(format!("unexpected token"))
            }
        }
    }

    pub fn parse(mut self) -> ExprResult<'c> {
        let mut expression_list: Vec<&Expression> = vec![];
        loop {
            match self.next_expression(self.empty_expression) {
                Ok(expression) => {
                    if self.is_empty_expression(expression) {
                        break;
                    }
                    expression_list.push(expression);
                }
                Err(e) => {
                    // TODO: handle recursion of parsers better
                    if e.contains("Parse error:") {
                        return Err(e);
                    }

                    // TODO: handle the parsed line better
                    // TODO: add the token parsed to error messages somehow
                    let (index, line) = self.lexer.get_current_line();
                    return Err(format!("Parse error: {}\n{}: {}", e, index, line));
                }
            }
        };
        if expression_list.len() > 0 {
            Ok(self.arena.alloc(Expression::Body(expression_list)))
        } else {
            Ok(self.empty_expression)
        }
    }
}
