use crate::token::Token;
use crate::types::Type;

type Expr<'a> = &'a mut Expression<'a>;

// TODO: change all enum values to have names
#[derive(Debug)]
pub enum Expression<'a> {
    Empty,
    Ident(String),
    Literal(String),
    Operation(Expr<'a>, Token, Expr<'a>),
    IfThen(Expr<'a>, Expr<'a>),
    IfElseThen(Expr<'a>, Expr<'a>, Expr<'a>),
    Body(Vec<Expr<'a>>),
    Declaration(
        Expr<'a>, /* expression */
        Type,     /* type */
        Expr<'a>, /* value */
    ),
    FunctionHeader(String /* name */, Expr<'a> /* variables */),
    Return(Expr<'a>),
}

impl Expression<'_> {
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
                left.get_tree_string(tabs + 2),
                right.get_tree_string(tabs + 2),
                Expression::get_formatted_tabs(tabs)
            ),
            Expression::Body(expressions) => {
                let mut str = "".to_string();
                let formatted_tabs = Expression::get_formatted_tabs(tabs);
                for expression in expressions {
                    str.push_str(&*format!(
                        "{1}\n  {1}({1}{0}\n  {1}),",
                        expression.get_tree_string(tabs + 2),
                        formatted_tabs
                    ));
                }
                format!("\n{1}[{0}\n{1}]", str, formatted_tabs)
            }
            Expression::IfThen(condition, then) => format!(
                "\n{2}if:\n  {2}condition: {0}\n  {2}then: {1}",
                condition.get_tree_string(tabs + 2),
                then.get_tree_string(tabs + 2),
                Expression::get_formatted_tabs(tabs)
            ),
            Expression::IfElseThen(condition, then, else_expr) => format!(
                "\n{3}if:\n  {3}condition: {0}\n  {3}else: {1}\n  {3}then: {2}",
                condition.get_tree_string(tabs + 2),
                then.get_tree_string(tabs + 2),
                else_expr.get_tree_string(tabs + 2),
                Expression::get_formatted_tabs(tabs)
            ),
            Expression::Declaration(expression, typ, value) => format!(
                "\n{3}declaration:\n  {3}expression: {0}\n  {3}type: {1}\n  {3}value: {2}",
                expression.get_tree_string(tabs + 2),
                typ,
                value.get_tree_string(tabs + 2),
                Expression::get_formatted_tabs(tabs)
            ),
            Expression::FunctionHeader(name, variables) => format!(
                "\n{2}function:\n  {2}name: {0}\n  {2}variables: {1}",
                name,
                variables.get_tree_string(tabs + 2),
                Expression::get_formatted_tabs(tabs)
            ),
            Expression::Return(expression) => format!(
                "\n{1}return: {0}",
                expression.get_tree_string(tabs + 2),
                Expression::get_formatted_tabs(tabs)
            ),
        }
    }
}

impl ToString for Expression<'_> {
    fn to_string(&self) -> String {
        self.get_tree_string(0)
    }
}
