use std::cell::RefCell;
use std::rc::Rc;

use generational_arena::Arena;
use generational_arena::Index;

use crate::token::Token;
use crate::types::Type;

lazy_static! {
    pub static ref EXPRESSION_ARENA : Rc<RefCell<Arena<Expression>>> = Arc::new(RefCell::new(Arena::new()));
    pub static ref EMPTY_EXPRESSION : Index = EXPRESSION_ARENA.insert(Expression::Empty);
}

// TODO: change all enum values to have names
#[derive(Debug)]
pub enum Expression {
    Empty,
    Ident(String),
    Literal(String),
    Operation(Index, Token, Index),
    IfThen(Index, Index),
    IfElseThen(Index, Index, Index),
    Body(Vec<Index>),
    Declaration(
        Index, /* expression */
        Type,  /* type */
        Index, /* value */
    ),
    FunctionHeader(String /* name */, Index /* variables */),
    Return(Index),
}

impl Expression {
    fn get_formatted_tabs(tabs: usize) -> String {
        let mut tab_str = "".to_string();
        for _ in (0..tabs).step_by(1) {
            tab_str.push_str("  ");
        }
        tab_str
    }

    fn get_tree_string(&self, tabs: usize) -> String {
        match self {
            Expression::Empty => "empty".to_string(),
            Expression::Ident(name) => format!(
                "\n{1}ident: {0}",
                name,
                Expression::get_formatted_tabs(tabs)
            ),
            Expression::Literal(name) => format!(
                "\n{1}literal: {0}",
                name,
                Expression::get_formatted_tabs(tabs)
            ),
            Expression::Operation(left, token, right) => format!(
                "\n{3}{0}:\n  {3}left: {1}\n  {3}right: {2}",
                token.value,
                Self::get_expression_string(left, tabs),
                Self::get_expression_string(right, tabs),
                Expression::get_formatted_tabs(tabs)
            ),
            Expression::Body(expressions) => {
                let mut str = "".to_string();
                let formatted_tabs = Expression::get_formatted_tabs(tabs);
                for expression in expressions {
                    str.push_str(&*format!(
                        "{1}\n  {1}({1}{0}\n  {1}),",
                        Self::get_expression_string(expression, tabs),
                        formatted_tabs
                    ));
                }
                format!("\n{1}[{0}\n{1}]", str, formatted_tabs)
            }
            Expression::IfThen(condition, then) => format!(
                "\n{2}if:\n  {2}condition: {0}\n  {2}then: {1}",
                Self::get_expression_string(condition, tabs),
                Self::get_expression_string(then, tabs),
                Expression::get_formatted_tabs(tabs)
            ),
            Expression::IfElseThen(condition, then, else_expr) => format!(
                "\n{3}if:\n  {3}condition: {0}\n  {3}else: {1}\n  {3}then: {2}",
                Self::get_expression_string(condition, tabs),
                Self::get_expression_string(then, tabs),
                Self::get_expression_string(else_expr, tabs),
                Expression::get_formatted_tabs(tabs)
            ),
            Expression::Declaration(expression, typ, value) => format!(
                "\n{3}declaration:\n  {3}expression: {0}\n  {3}type: {1}\n  {3}value: {2}",
                Self::get_expression_string(expression, tabs),
                typ,
                Self::get_expression_string(value, tabs),
                Expression::get_formatted_tabs(tabs)
            ),
            Expression::FunctionHeader(name, variables) => format!(
                "\n{2}function:\n  {2}name: {0}\n  {2}variables: {1}",
                name,
                Self::get_expression_string(variables, tabs),
                Expression::get_formatted_tabs(tabs)
            ),
            Expression::Return(expression) => format!(
                "\n{1}return: {0}",
                Self::get_expression_string(expression, tabs),
                Expression::get_formatted_tabs(tabs)
            ),
        }
    }

    fn get_expression_string(index: &Index, current_tabs: usize) -> String {
        if let Some(expr) = EXPRESSION_ARENA.get(*index) {
            expr.get_tree_string(current_tabs + 2)
        } else {
            "deleted".to_string()
        }
    }

    pub fn from_index_to_string(index: &Index) -> String {
        if let Some(expr) = EXPRESSION_ARENA.get(*index) {
            expr.to_string()
        } else {
            "deleted".to_string()
        }
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        self.get_tree_string(0)
    }
}
