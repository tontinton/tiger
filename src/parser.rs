use typed_arena::Arena;

use crate::ast::Expression;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use crate::types::Type;

type Expr<'b> = &'b mut Expression<'b>;
type ExprResult<'b> = Result<&'b mut Expression<'b>, String>;

pub struct Parser<'a, 'b> {
    lexer: &'a mut Lexer,
    arena: &'b Arena<Expression<'b>>,
    empty_expression: Expr<'b>,
    done_parsing: bool,
    stop_at: Option<char>,
    separate_at: char,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn new(lexer: &'a mut Lexer,
               arena: &'b Arena<Expression<'b>>,
               empty_expression: Expr<'b>) -> Self {
        Self {
            lexer,
            arena,
            empty_expression,
            done_parsing: false,
            stop_at: None,
            separate_at: ';',
        }
    }

    fn eat_token(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    fn is_empty_expression(&self, expression: Expr<'b>) -> bool {
        expression as *const _ == self.empty_expression as *const _
    }

    fn get_special_char_expression(&mut self, prev: Expr<'b>, token: Token) -> ExprResult<'b> {
        let c = token.value.as_bytes()[0] as char;
        if c == self.separate_at {
            return Ok(prev);
        }

        if let Some(stop_char) = self.stop_at {
            if stop_char == c {
                self.done_parsing = true;
                return Ok(prev);
            }
        }

        match c {
            '=' => {
                if self.is_empty_expression(prev) {
                    return Err("assignment: no expression before `=`".to_string());
                }
                match &(*prev) {
                    Expression::Literal(_) => Ok(()),
                    Expression::Ident(_) => Ok(()),
                    Expression::Operation(_left, _token, _right) => Ok(()),
                    _ => {
                        Err("assignment: the previous expression must either be a literal or an operation".to_string())
                    }
                }?;

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
            '{' => {
                // TODO: match on if statements and functions here
                if self.is_empty_expression(prev) {
                    self.next_body()
                } else {
                    Err("`{` can only come after an empty expression".to_string())
                }
            }
            '(' => {
                match prev {
                    Expression::Ident(name) => {
                        let variables = self.next_header()?;
                        self.next_expression(self.arena.alloc(Expression::FunctionHeader(name.clone(), variables)))
                    }
                    _ => {
                        self.next_header()
                    }
                }
            }
            _ => {
                Err(format!("special: the character '{}' is out of place", c))
            }
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

    fn get_operation_expression(&mut self, prev: Expr<'b>, token: Token) -> ExprResult<'b> {
        let subtree = self.next_expression(self.empty_expression)?;
        if self.is_empty_expression(subtree) {
            return Err(format!("operation: no expression found after {}", token.value));
        }

        let priority = Self::get_operation_priority(&token);

        if self.is_empty_expression(prev) {
            Err(format!("operation: no expression found before: {}", token.value))
        } else {
            if let Expression::Operation(left, subtree_token, right) = subtree {
                let subtree_priority = Self::get_operation_priority(&subtree_token);
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

    // TODO: fix this mess of a function
    fn get_variable_declaration_expression(&mut self) -> ExprResult<'b> {
        if let Some(name_token) = self.eat_token() {
            match name_token.typ {
                TokenType::Symbol => {
                    if let Some(colon_token) = self.eat_token() {
                        match colon_token.typ {
                            TokenType::Colon => {
                                if let Some(type_token) = self.eat_token() {
                                    match type_token.typ {
                                        TokenType::Symbol => {
                                            let variable = self.arena.alloc(Expression::Ident(name_token.value));
                                            let value = self.next_expression(variable)?;
                                            if self.is_empty_expression(value) {
                                                Err("`[name] : [type] = [expression]`, no [expression] found".to_string())
                                            } else {
                                                match value {
                                                    Expression::Operation(_left, _token, right) => {
                                                        let typ = Type::Undetermined{name: type_token.value};
                                                        Ok(self.arena.alloc(Expression::Declaration(variable, typ, right)))
                                                    }
                                                    _ => Err("`[name] : [type] = [expression]`, the [expression] must be an assignment".to_string())
                                                }
                                            }
                                        }
                                        _ =>  Err("`[name] : [type]`, [type] given after `:` is not a valid symbol".to_string())
                                    }
                                } else {
                                    Err("`[name] : [type]`, expression ended prematurely, [type] not found".to_string())
                                }
                            }
                            TokenType::Walrus => {
                                let value = self.next_expression(self.empty_expression)?;
                                if self.is_empty_expression(value) {
                                    Err("`[name] := [expression]`, no [expression] found".to_string())
                                } else {
                                    let variable = self.arena.alloc(Expression::Ident(name_token.value));
                                    Ok(self.arena.alloc(Expression::Declaration(variable, Type::Auto, value)))
                                }
                            }
                            _ =>  Err("`[name] : [type]`, `:` or `:=` did not come after [name]".to_string())
                        }
                    } else {
                        Err("`[name] : [type]`, expression ended prematurely, nothing came after [name]".to_string())
                    }
                }
                _ =>  Err("`[name] : [type]`, the [name] given is not a valid symbol".to_string())
            }
        } else {
            Err("`[name] : [type]`, expression ended prematurely, [name] not found".to_string())
        }
    }

    fn next_scope(&mut self, separate_at: char, stop_at: char) -> ExprResult<'b> {
        let mut parser = Parser::new(self.lexer, self.arena, self.empty_expression);
        parser.separate_at = separate_at;
        parser.stop_at = Some(stop_at);
        parser.parse()
    }

    fn next_header(&mut self) -> ExprResult<'b> {
        self.next_scope(',', ')')
    }

    fn next_body(&mut self) -> ExprResult<'b> {
        self.next_scope(';', '}')
    }

    fn next_expression_until_char(&mut self, stop_at: char) -> ExprResult<'b> {
        let old_separate_at = self.separate_at;

        self.separate_at = stop_at;
        let expression = self.next_expression(self.empty_expression);
        self.separate_at = old_separate_at;

        expression
    }

    fn next_expression(&mut self, prev: Expr<'b>) -> ExprResult<'b> {
        let token = match self.eat_token() {
            Some(x) => x,
            None => {
                self.done_parsing = true;
                return Ok(self.empty_expression);
            }
        };

        match token.typ {
            TokenType::Special => self.get_special_char_expression(prev, token),
            TokenType::Operation => self.get_operation_expression(prev, token),
            TokenType::Symbol => self.next_expression(self.arena.alloc(Expression::Ident(token.value))),
            TokenType::If => {
                let condition = self.next_expression_until_char('{')?;
                if self.is_empty_expression(condition) {
                    return Err("if: empty condition".to_string());
                }
                let then = self.next_body()?;
                if self.is_empty_expression(condition) {
                    return Err("if: empty body".to_string());
                }
                let if_expr = self.arena.alloc(Expression::IfThen(condition, then));

                if let Some(next_token) = self.lexer.peek() {
                    match next_token.typ {
                        TokenType::Else => self.next_expression(if_expr),
                        _ => Ok(if_expr)
                    }
                } else {
                    Ok(if_expr)
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
                            return Ok(prev);
                        }
                        match else_expr {
                            Expression::IfThen(_, _) => Ok(self.arena.alloc(Expression::IfElseThen(condition, else_expr, then))),
                            Expression::Body(_) => Ok(self.arena.alloc(Expression::IfElseThen(condition, else_expr, then))),
                            _ => Err("`{` was not found after `else`".to_string())
                        }
                    }
                    _ => {
                        Err("`else` can only be after an `if` block".to_string())
                    }
                }
            }
            TokenType::Number => {
                self.next_expression(self.arena.alloc(Expression::Literal(token.value)))
            }
            TokenType::Colon => {
                match prev {
                    Expression::Ident(name) => {
                        if let Some(type_token) = self.eat_token() {
                            match type_token.typ {
                                TokenType::Symbol => {
                                    let variable = self.arena.alloc(Expression::Ident(name.clone()));
                                    let typ = Type::Undetermined {name: type_token.value};
                                    self.next_expression(self.arena.alloc(Expression::Declaration(variable, typ, self.empty_expression)))
                                }
                                _ => {
                                    Err("`[name] : [type]`, [type] given after `:` is not a valid symbol".to_string())
                                }
                            }
                        } else {
                            Err("`[name] : [type]`, expression ended prematurely, [type] not found".to_string())
                        }
                    }
                    _ => Err("`:` must only come after a variable name".to_string())
                }
            }
            TokenType::Let => {
                if !self.is_empty_expression(prev) {
                    return Err("`let [name] : [type]`, `let` must at the beginning of the expression".to_string());
                }

                self.get_variable_declaration_expression()
            }
            TokenType::Func => {
                if !self.is_empty_expression(prev) {
                    return Err("`fn` must at the beginning of the expression".to_string());
                }
                self.next_expression(self.empty_expression)
            }
            TokenType::SmallArrow => {
                match prev {
                    Expression::FunctionHeader(_name, _variables) => {
                        let type_token = if let Some(next_token) = self.eat_token() {
                            match next_token.typ {
                                TokenType::Symbol => {
                                    Ok(next_token)
                                }
                                _ => Err("the type given after `->` is not valid")
                            }
                        } else {
                            Err("no token found after `->`")
                        }?;

                        let body = self.next_expression(self.empty_expression)?;
                        match body {
                            Expression::Body(_) => {
                                let typ = Type::Undetermined {name: type_token.value};
                                Ok(self.arena.alloc(Expression::Declaration(prev, typ, body)))
                            }
                            _ => Err("after `->` there must be a new scope declared by `{`".to_string())
                        }
                    }
                    _ => Err("`->` can only come after a function declaration".to_string())
                }
            }
            TokenType::Return => {
                if !self.is_empty_expression(prev) {
                    return Err("`return` must at the beginning of the expression".to_string());
                }

                let next_expression = self.next_expression(self.empty_expression)?;
                Ok(self.arena.alloc(Expression::Return(next_expression)))
            }
            _ => {
                Err(format!("unexpected token: {}", token.value))
            }
        }
    }

    pub fn parse(mut self) -> ExprResult<'b> {
        let mut expressions: Vec<&Expression> = vec![];
        loop {
            match self.next_expression(self.empty_expression) {
                Ok(expression) => {
                    if self.is_empty_expression(expression) && self.done_parsing {
                        break;
                    }
                    expressions.push(expression);
                    if self.done_parsing {
                        break;
                    }
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
        if expressions.len() > 0 {
            Ok(self.arena.alloc(Expression::Body(expressions)))
        } else {
            Ok(self.empty_expression)
        }
    }
}
