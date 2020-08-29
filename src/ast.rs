use crate::token::Token;

type Expr<'a> = &'a Expression<'a>;

// TODO: change all enum values to have names
pub enum Expression<'a> {
    Literal(Token),
    Operation(Expr<'a>, Token, Expr<'a>),
    IfThen(Expr<'a>, Expr<'a>),
    IfElseThen(Expr<'a>, Expr<'a>, Expr<'a>),
    Body(Vec<Expr<'a>>),
    VariableDeclaration(Expr<'a> /* variable */, Expr<'a> /* type */, Expr<'a> /* value */),
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
            Expression::Literal(token) => token.value.clone(),
            Expression::Operation(left, token, right) => {
                format!("\n{3}{0}:\n  {3}left: {1}\n  {3}right: {2}",
                        token.value,
                        left.get_tree_string(tabs + 2),
                        right.get_tree_string(tabs + 2),
                        Expression::get_formatted_tabs(tabs))
            }
            Expression::Body(expressions) => {
                let mut str = "".to_string();
                let formatted_tabs = Expression::get_formatted_tabs(tabs);
                for expression in expressions {
                    str.push_str(&*format!("{1}\n  {1}({1}{0}\n  {1}),", expression.get_tree_string(tabs + 2), formatted_tabs));
                }
                format!("\n{1}[{0}\n{1}]", str, formatted_tabs)
            }
            Expression::IfThen(condition, then) => {
                format!("\n{2}if:\n  {2}condition: {0}\n  {2}then: {1}",
                        condition.get_tree_string(tabs + 2),
                        then.get_tree_string(tabs + 2),
                        Expression::get_formatted_tabs(tabs))
            }
            Expression::IfElseThen(condition, then, else_expr) => {
                format!("\n{3}if:\n  {3}condition: {0}\n  {3}then: {1}\n  {3}else: {2}",
                        condition.get_tree_string(tabs + 2),
                        then.get_tree_string(tabs + 2),
                        else_expr.get_tree_string(tabs + 2),
                        Expression::get_formatted_tabs(tabs))
            }
            Expression::VariableDeclaration(variable, typ, value) => {
                format!("\n{3}declaration:\n  {3}name: {0}\n  {3}type: {1}\n  {3}value: {2}",
                        variable.get_tree_string(tabs + 2),
                        typ.get_tree_string(tabs + 2),
                        value.get_tree_string(tabs + 2),
                        Expression::get_formatted_tabs(tabs))
            }
        }
    }
}

impl ToString for Expression<'_> {
    fn to_string(&self) -> String {
        self.get_tree_string(0)
    }
}
